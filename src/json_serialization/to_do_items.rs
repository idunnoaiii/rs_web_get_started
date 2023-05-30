use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use crate::to_do::{to_do_factory, ItemTypes};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::schema::to_do;
use diesel::prelude::*;

#[derive(Serialize)]
pub struct TodoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl TodoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> TodoItems {
        let mut pending_items: Vec<Base> = Vec::new();
        let mut done_items: Vec<Base> = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Done(item) => done_items.push(item.super_struct),
                ItemTypes::Pending(item) => pending_items.push(item.super_struct),
            };
        }

        let pending_count = pending_items.len() as i8;
        let done_count = done_items.len() as i8;

        return TodoItems {
            pending_items,
            done_items,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }

    #[allow(unused)]
    fn get_state_json() -> TodoItems {
        let state = read_file("./state.json");
        let mut array_buffer = Vec::new();
        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);
            array_buffer.push(item);
        }
        return TodoItems::new(array_buffer);
    }

    pub fn get_state() -> TodoItems {
        let connection = establish_connection();
        let mut array_buffer = Vec::new();

        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();

        for item in items {
            let status = TaskStatus::from_string(item.status);
            let new_item = to_do_factory(&item.title, status);
            array_buffer.push(new_item);
        }

        return TodoItems::new(array_buffer);
    }
}

impl Responder for TodoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
}

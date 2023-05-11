use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use crate::to_do::{ItemTypes, to_do_factory};
use actix_web::http::header::ContentType;
use actix_web::{Responder, HttpResponse};
use actix_web::body::BoxBody;
use serde::Serialize;

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

    pub fn get_state() -> TodoItems {
        let state = read_file("./state.json");

        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);
            array_buffer.push(item);
        }

        return TodoItems::new(array_buffer);
    }
}


impl Responder for TodoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {

        let body = serde_json::to_string(&self).unwrap();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
    
}

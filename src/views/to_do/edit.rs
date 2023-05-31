use actix_web::{web, HttpResponse};

use crate::process::process_input;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use crate::{
    json_serialization::to_do_item::TodoItem, json_serialization::to_do_items::TodoItems,
    state::read_file,
};

use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::schema::to_do;
use diesel::prelude::*;

#[allow(unused)]
async fn edit_json(to_do_item: web::Json<TodoItem>) -> HttpResponse {
    let state = read_file("./state.json");
    let status: TaskStatus;

    match state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());

    if &status.stringify()
        == &TaskStatus::from_string(to_do_item.status.as_str().to_string()).stringify()
    {
        return HttpResponse::Ok().json(TodoItems::get_state());
    }

    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(TodoItems::get_state());
}

pub async fn edit(to_do_item: web::Json<TodoItem>) -> HttpResponse {
    let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(to_do_item.title.as_str().to_string()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        return HttpResponse::NotFound().json("{} is not found");
    }

    let _ = diesel::update(to_do::table.filter(to_do::columns::title.eq(&to_do_item.title)))
        .set(to_do::columns::status.eq("DONE"))
        .execute(&connection);

    return HttpResponse::Ok().json(TodoItems::get_state());
}

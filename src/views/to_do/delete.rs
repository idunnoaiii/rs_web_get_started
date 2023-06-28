use crate::diesel;
use crate::json_serialization::to_do_item::TodoItem;
use crate::json_serialization::to_do_items::TodoItems;
use crate::jwt::JwtToken;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::schema::to_do;
use crate::models::item::item::Item;

pub async fn delete(to_do_item: web::Json<TodoItem>, _token: JwtToken ) -> HttpResponse {
    let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(to_do_item.title.as_str().to_string()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        return HttpResponse::NotFound().json("{} is not found");
    }

    let _ = diesel::delete(to_do::table.filter(to_do::columns::title.eq(&to_do_item.title)))
        .execute(&connection);

    return HttpResponse::Ok().json(TodoItems::get_state());
}
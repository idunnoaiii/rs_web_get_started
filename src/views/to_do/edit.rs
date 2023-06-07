use crate::database::DB;
use crate::diesel;
use crate::jwt::JwtToken;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::{json_serialization::to_do_item::TodoItem, json_serialization::to_do_items::TodoItems};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn edit(to_do_item: web::Json<TodoItem>, _token: JwtToken, db: DB) -> HttpResponse {
    //let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(to_do_item.title.as_str().to_string()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        return HttpResponse::NotFound().json("{} is not found");
    }

    let _ = diesel::update(to_do::table.filter(to_do::columns::title.eq(&to_do_item.title)))
        .set(to_do::columns::status.eq("DONE"))
        .execute(&db.connection);

    return HttpResponse::Ok().json(TodoItems::get_state());
}

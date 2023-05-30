use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::process::process_input;
use crate::diesel;
use diesel::prelude::*;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use crate::json_serialization::to_do_items::TodoItems;
use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;

#[allow(unused)]
pub async fn create_json(req: HttpRequest) -> String {
    let state = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();

    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);
    return format!("{} created", title); 
}

pub async fn create(req: HttpRequest) -> HttpResponse {
    let title = req.match_info().get("title").unwrap().to_string();
    let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_item = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_item)
            .execute(&connection);
    }

    return HttpResponse::Ok().json(TodoItems::get_state());
}
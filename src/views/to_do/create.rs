use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_items::TodoItems;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use diesel::prelude::*;
use crate::jwt::JwtToken;

pub async fn create(req: HttpRequest, _token: JwtToken) -> HttpResponse {
    let title = req.match_info().get("title").unwrap().to_string();
    let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_item = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_item)
            .execute(&connection);
    }

    return HttpResponse::Ok().json(TodoItems::get_state());
}

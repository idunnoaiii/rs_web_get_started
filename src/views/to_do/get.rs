use crate::json_serialization::to_do_items::TodoItems;
use actix_web::{HttpRequest, Responder};

pub async fn get(_req: HttpRequest) -> impl Responder {
    return TodoItems::get_state();
}

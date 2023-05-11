use crate::json_serialization::to_do_items::TodoItems;
use crate::{
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};
use actix_web::{web, HttpRequest, Responder};
use serde_json::{Map, Value};

pub async fn get(_req: HttpRequest) -> impl Responder {
    return TodoItems::get_state();
}

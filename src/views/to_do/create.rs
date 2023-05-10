use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;


use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::process::process_input;


pub async fn create(req: HttpRequest) -> String {
    let state = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();

    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);
    return format!("{} created", title); 
}
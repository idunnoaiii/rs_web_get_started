use actix_web::web::{self, get};

mod items;

pub fn app_view_factory(app: &mut web::ServiceConfig) {
    app.route("/", get().to(items::items));
}
use actix_web::web::{ServiceConfig, scope, get, put, post, delete};

mod create;
mod get;
mod edit;
mod delete;

pub fn to_do_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", get().to(create::create))
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("edit", put().to(edit::edit))
            .route("delete", delete().to(delete::delete))
    );
}
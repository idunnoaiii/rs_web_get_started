use actix_web::web::{ServiceConfig, scope, get, post};

mod create;

pub fn to_do_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", get().to(create::create))
            .route("create/{title}", post().to(create::create))
    );
}
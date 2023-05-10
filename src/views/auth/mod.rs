mod login;
mod logout;

use actix_web::web::{ServiceConfig, get, scope};

use self::{login::login, logout::logout};

pub fn auth_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", get().to(login))
            .route("logout", get().to(logout))
    );
}

mod auth;
mod to_do;
mod app;

use actix_web::web::ServiceConfig;

use auth::auth_view_factory;

use self::{to_do::to_do_view_factory, app::app_view_factory};

pub fn views_factory(app: &mut ServiceConfig) {
    auth_view_factory(app);
    to_do_view_factory(app);
    app_view_factory(app);
}

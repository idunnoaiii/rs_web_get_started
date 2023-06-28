use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use crate::database::DB;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user_rq: web::Json<NewUserSchema>, db: DB) -> impl Responder {
    let new_user = NewUser::new(
        new_user_rq.username.clone(),
        new_user_rq.email.clone(),
        new_user_rq.password.clone(),
    );

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&db.connection);

    return match insert_result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::Conflict()
    };
}
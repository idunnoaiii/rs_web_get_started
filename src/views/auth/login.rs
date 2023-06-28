use std::collections::HashMap;
use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder, HttpMessage};
use serde_json::json;
use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::jwt::JwtToken;

pub async fn login(credential: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credential.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(credential.username.clone()))
        .load::<User>(&db.connection).unwrap();

    dbg!("{}", &credential.username);

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    }

    if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    return match users.get(0).unwrap().verify(password) {
        true => {
            let token = JwtToken::new(users[0].id);
            let raw_token = token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);
            HttpResponse::Ok().json(body)
        }
        false => {
            HttpResponse::Unauthorized().await.unwrap()
        }
    }
}

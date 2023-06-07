use crate::config::Config;
use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>>{
    DBCONNECTION.db_connection.get().unwrap()
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => return ok(DB { connection }),
            Err(_) => {
                return err(ErrorServiceUnavailable(
                    "could not make connection to the database",
                ))
            }
        }
    }
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        return DbConnection {
            db_connection: PgPool::builder()
                .max_size(1)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection_pool"),
        };
    };
}

mod errors;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use models::UserCounter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Users {
    users: Vec<models::User>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let init_users = Arc::new(RwLock::new(Users { users: vec![] }));
    let init_ctr = Arc::new(RwLock::new(UserCounter { counter: 1 }));
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec![
                "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH",
            ])
            .allowed_headers(vec![
                "authorization",
                "Origin",
                "X-Requested-With",
                "Content-Type",
                "Accept",
                "X-Access-Token",
                "x-api-key",
            ])
            .send_wildcard()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(init_users.clone()))
            .app_data(web::Data::new(init_ctr.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(handlers::put_user)
            .service(handlers::get_users)
            .service(handlers::delte_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use redis::Commands;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Response {
    views: i32,
}

async fn view_counter() -> impl Responder {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL");
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();

    let count: i32 = con.incr("github_profile_views", 1).unwrap();
    HttpResponse::Ok().json(Response { views: count })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| {
        App::new()
            .route("/views", web::get().to(view_counter))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}

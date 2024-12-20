use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use redis::Commands;
use std::env;

async fn view_counter() -> impl Responder {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL");
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();

    let count: i32 = con.incr("github_profile_views", 1).unwrap();
    HttpResponse::Ok().json(format!("Visitas: {}", count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/views", web::get().to(view_counter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

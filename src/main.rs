use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenvy::dotenv;
use std::env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "UP",
        "message": "Server is running smoothly"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    // Ambil nilai PORT dari environment
    let port = env::var("PORT")
        .expect("PORT environment variable not set");

    let address = format!("127.0.0.1:{}", port);

    println!("🚀 Server is running at http://{}", address);
    println!("🔥 Press Ctrl+C to stop the server");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check)) // Route untuk health check
    })
    .bind(address)? // Bind ke alamat yang ditentukan
    .run()
    .await
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};



#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;
    println!("Starting server at {ip}:{port}");

    const DB_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlx";

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::scope("/api")
                .service(hello)
                .service(user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[derive(Serialize, Deserialize)]
struct UserReq {
    id: u64,
}

#[post("/user")]
async fn user(user_id: web::Json<UserReq>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = user_id.into_inner().id;
    println!("User id: {user_id}");
    
    let user: sqlx::Result<User> = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = ?",
        user_id,
    ).fetch_one(&app_state.pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String
}

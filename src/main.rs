use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};



#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 12345;
    println!("Starting server at http://{ip}:{port}");

    const DB_URL: &str = "postgres://user:password@127.0.0.1:5432/sqlx";

    let pool: PgPool = PgPoolOptions::new()
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
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[derive(Serialize, Deserialize)]
struct UserReq {
    id: i64,
}

#[post("/user")]
async fn user(user_id: web::Json<UserReq>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = user_id.into_inner().id;
    println!("User id: {user_id}");
    
    let user: sqlx::Result<User> = sqlx::query_as!(
        User,
        "SELECT id, username, passwd, email FROM users WHERE id = $1",
        user_id,
    ).fetch_one(&app_state.pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    passwd: Vec<u8>,
    email: String
}

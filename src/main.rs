pub mod app;
use actix_web::{web, App, HttpServer};

use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

mod api;
use api::{api_root, auth_delete, auth_login, auth_logout, auth_passwd, auth_refresh, create_user};


use app::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = "127.0.0.1";
    let port = 12345;
    println!("Starting server at http://{ip}:{port}");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable missing.");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    //let user = sqlx::Result<User> = api::create_user("admin", "YWRtaW4=", "admin@example.com", &pool);
    let user = create_user("admin", "YWRtaW4=", "admin@example.com", &pool).await;
    println!("{user:?}");

    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("/api/v1").service(api_root).service(
                    web::scope("/auth")
                        .service(auth_login)
                        .service(auth_refresh)
                        .service(auth_logout)
                        .service(auth_passwd)
                        .service(auth_delete),
                ),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

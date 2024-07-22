use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::app::AppState;

#[get("/")]
pub async fn api_root() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    username: String,
    passwd: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthToken {
    token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub passwd: Vec<u8>,
    pub email: String
}


#[post("/login")]
pub async fn auth_login(auth_req: web::Json<AuthRequest>, app_state: web::Data<AppState>) -> impl Responder {
    let auth_req: AuthRequest = auth_req.into_inner();
    println!("Auth request: {auth_req:#?}");
    
    let user: sqlx::Result<User> = sqlx::query_as!(
        User,
        "SELECT id, username, passwd, email FROM users WHERE username = $1 AND passwd = $2",
        auth_req.username, // $1
        auth_req.passwd.as_bytes() // $2
    ).fetch_one(&app_state.pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => {
            println!("User not found");
            HttpResponse::Forbidden().into()
        },
    }
}

#[post("/refresh")]
pub async fn auth_refresh() -> impl Responder {
    HttpResponse::Ok().body("Refresh TODO")
}

#[post("/logout")]
pub async fn auth_logout() -> impl Responder {
    HttpResponse::Ok().body("Logout TODO")
}

#[post("/passwd")]
pub async fn auth_passwd() -> impl Responder {
    HttpResponse::Ok().body("password change TODO")
}

#[post("/delete")]
pub async fn auth_delete() -> impl Responder {
    HttpResponse::Ok().body("deletion TODO")
}

pub async fn create_user(username: impl Into<String>, passwd: impl Into<String>, email: impl Into<String>, pool: &PgPool) -> sqlx::Result<User> {
    let password = passwd.into();
    let user: sqlx::Result<User> = sqlx::query_as!(
        User,
        "INSERT INTO users(username, passwd, email) VALUES ($1, $2, $3) RETURNING id, username, passwd, email",
        username.into(),
        password.as_bytes(),
        email.into()
    ).fetch_one(pool).await;
    user
}

use actix_web::{HttpServer, App, Responder, HttpResponse, get, post, error, web::{Data, Json, Path}};
use serde::{Serialize, Deserialize};
use tokio_postgres::{self, NoTls, Error};
use tokio;
use sqlx::{Postgres, postgres::PgPoolOptions, Pool, self, FromRow};
use postgres::{Client};


pub struct AppState{
    db : Pool<Postgres>
}
#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct InsertUserData{
    name: String,
    email: String,
    address: String
}
#[derive(Serialize, FromRow)]
pub struct UserData{
    id: i32,
    name: String,
    email: String,
    address: String
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let pg =PgPoolOptions::new().max_connections(5).connect("postgresql://postgres:sanjay@localhost/Demo").await.expect("Error while connecting.");
    // let(client, connection) = tokio_postgres::connect("postgresql://postgres:sanjay@localhost/Demo", NoTls)
    HttpServer::new(move|| {
        App::new()
        .app_data(Data::new(AppState {db: pg.clone()}))
        .service(home)
        .service(get_data)
        .service(select_data)
        .service(delete_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder{
    HttpResponse::Ok().body("Hey there!")
}

#[get("/user")]
async fn get_data(state: Data<AppState>) -> impl Responder{
    match sqlx::query_as::<_, UserData>("SELECT id, name, email, address FROM user_info")
    .fetch_all(&state.db).await {
        Ok(v) => {
            HttpResponse::Ok().json(v)
            },
        Err(_) => HttpResponse::InternalServerError().body("Data not found.")
    }
}

#[get("/user/{id}")]
async fn select_data(state: Data<AppState>, id: Path<i32>) -> impl Responder{
    let id: i32 = id.into_inner();
    match sqlx::query_as::<_, UserData>("SELECT id, name, email, address FROM user_info WHERE id = $1")
    .bind(id).fetch_all(&state.db).await {
        Ok(v) => {
            HttpResponse::Ok().json(v)
            },
        Err(_) => HttpResponse::InternalServerError().body("No user found.")
    }
}

#[get("/delete/{id}")]
async fn delete_data(state: Data<AppState>, id: Path<i32>) -> impl Responder{
    let id: i32 = id.into_inner();
    match sqlx::query_as::<_, UserData>("DELETE FROM user_info WHERE id = $1")
    .bind(id).fetch_all(&state.db).await {
        Ok(_) => {
            HttpResponse::Ok().body("User deleted.")
            },
        Err(_) => HttpResponse::InternalServerError().body("No user found.")
    }
}

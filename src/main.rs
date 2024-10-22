use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Tx {
    id: i64,
    ip: Option<String>,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


async fn get_transaction(pool: web::Data<SqlitePool>) -> impl Responder {
    let users = sqlx::query_as!(Tx, "SELECT id, ip FROM user_transactions")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(users)

}

async fn insert_test_data(pool: web::Data<SqlitePool>, tx: web::Json<Tx>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO user_transactions (ip) VALUES (?1)", tx.ip)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add user"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite://my_database.db").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)            
            .route("/tx", web::get().to(get_transaction))
            .route("/tx", web::post().to(insert_test_data))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
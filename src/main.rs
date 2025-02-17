use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Page {
    id: Option<i64>,
    title: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageResponse {
    id: i64,
    title: String,
    content: String,
}

async fn create_page(
    pool: web::Data<SqlitePool>,
    page: web::Json<Page>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO pages (title, content) VALUES (?, ?)",
        page.title,
        page.content
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            let id = result.last_insert_rowid();
            HttpResponse::Ok().json(PageResponse {
                id,
                title: page.title.clone(),
                content: page.content.clone(),
            })
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_pages(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as!(
        PageResponse,
        "SELECT id, title, content FROM pages"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(pages) => HttpResponse::Ok().json(pages),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_page(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let id = id.into_inner();
    let result = sqlx::query_as!(
        PageResponse,
        "SELECT id, title, content FROM pages WHERE id = ?",
        id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(page)) => HttpResponse::Ok().json(page),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn update_page(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    page: web::Json<Page>,
) -> impl Responder {
    let id = id.into_inner();
    let result = sqlx::query!(
        "UPDATE pages SET title = ?, content = ? WHERE id = ?",
        page.title,
        page.content,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(PageResponse {
                    id,
                    title: page.title.clone(),
                    content: page.content.clone(),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_page(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let id = id.into_inner();
    let result = sqlx::query!(
        "DELETE FROM pages WHERE id = ?",
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Create SQLite database and run migrations
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Create tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS pages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/pages", web::post().to(create_page))
                    .route("/pages", web::get().to(get_pages))
                    .route("/pages/{id}", web::get().to(get_page))
                    .route("/pages/{id}", web::put().to(update_page))
                    .route("/pages/{id}", web::delete().to(delete_page))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

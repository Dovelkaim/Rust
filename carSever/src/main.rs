use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Car {
    id: i64,
    name: String,
    color: String,
    year: i32,
}

#[derive(Debug, Deserialize)]
struct CreateCarRequest {
    name: String,
    color: String,
    year: i32,
}

async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cars (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            year INTEGER NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

#[post("/car")]
async fn create_car(
    pool: web::Data<SqlitePool>,
    car: web::Json<CreateCarRequest>,
) -> impl Responder {
    match sqlx::query("INSERT INTO cars (name, color, year) VALUES (?, ?, ?)")
        .bind(&car.name)
        .bind(&car.color)
        .bind(car.year)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            let new_car = Car {
                id: result.last_insert_rowid(),
                name: car.name.clone(),
                color: car.color.clone(),
                year: car.year,
            };
            HttpResponse::Created().json(new_car)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {e}")),
    }
}

#[get("/car")]
async fn get_cars(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, name, color, year FROM cars ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let cars: Vec<Car> = rows
                .iter()
                .map(|row| Car {
                    id: row.get("id"),
                    name: row.get("name"),
                    color: row.get("color"),
                    year: row.get("year"),
                })
                .collect();

            HttpResponse::Ok().json(cars)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {e}")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await.expect("DB init failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_car)
            .service(get_cars)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
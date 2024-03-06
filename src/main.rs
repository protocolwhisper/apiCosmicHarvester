use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

mod db;
use db::reading::get_lowest_priced_listing_for_address;

// Assuming `db` module and `get_lowest_priced_listing_for_address` function are defined correctly elsewhere

async fn get_floor_price(pool: web::Data<sqlx::Pool<sqlx::Postgres>>, nft_address: web::Path<String>) -> impl Responder {
    match get_lowest_priced_listing_for_address(&pool, &nft_address).await {
        Ok(Some(listing)) => HttpResponse::Ok().json(listing),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Example index function for demonstration
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Health is ok :)!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(index)) // Changed to `get` for the demonstration
            .route("/pallet_floorprice_collection/{nft_address}", web::get().to(get_floor_price))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

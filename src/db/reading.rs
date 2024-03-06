use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error, FromRow};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Listing {
    // Ensure these fields match your database schema
    listing_id: i64,
    nft_address: String,
    min_price: String, // Adjust the type according to your schema
    // Add other fields as necessary
}

pub async fn get_lowest_priced_listing_for_address(pool: &sqlx::Pool<sqlx::Postgres>, nft_address: &str) -> Result<Option<Listing>, sqlx::Error> {
    let result = sqlx::query_as!(
        Listing,
        "SELECT listing_id, nft_address, min_price FROM \"public\".\"palletlistings\"
         WHERE listed = true AND nft_address = $1
         ORDER BY min_price ASC
         LIMIT 1",
        nft_address
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}

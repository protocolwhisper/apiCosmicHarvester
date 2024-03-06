use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::{Error, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FloorPrice {
    listing_id: i64,
    nft_address: String,
    min_price: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PalletListing {
    pub listing_id: i64,
    pub nft_owner: String,
    pub nft_address: String,
    pub token_id: String,
    pub min_price: String,
    pub block_height: String,
    pub txhash: String,
    pub listed: bool,
}
pub async fn get_lowest_priced_listing_for_address(
    pool: &sqlx::Pool<sqlx::Postgres>,
    nft_address: &str,
) -> Result<Option<FloorPrice>, sqlx::Error> {
    let result = sqlx::query_as!(
        FloorPrice,
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

pub async fn get_listings_for_address(
    pool: &sqlx::Pool<sqlx::Postgres>,
    nft_address: &str,
) -> Result<Vec<PalletListing>, sqlx::Error> {
    let result = sqlx::query_as!(
        PalletListing,
        "SELECT * FROM \"public\".\"palletlistings\"
         WHERE listed = true AND nft_address = $1",
        nft_address
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

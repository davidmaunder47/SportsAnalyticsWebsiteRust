use serde::{Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::backend::DbJsonError;
use crate::shared::{DBStructSupport};

#[derive(Debug)]
pub struct DBPool {
    pub pg_pool: Pool<Postgres>,
    pub database: String
}
//I think its better if the structs implement the query_db instead of the db connection iteself
//therefore we dont have to force a enum return type we can just use the struct to get s vec of struct
//via a static function for each specific sturct
pub trait DBManager : Sized {
    type SportsType;
    async fn new_connection_to_db(database: &str) -> Result<Self, sqlx::Error>
    where
        Self: std::convert::From<DBPool>
    {
        let url = format!("postgres://postgres:Abcxyz007@@@@localhost:5433/{database}");
        let pool_conn: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url.as_str())
            .await?;

        let dbpool : DBPool = DBPool {pg_pool: pool_conn, database: database.to_string()};
        Ok(Self::from(dbpool))
    }
    async fn query_db<T>(&self) -> sqlx::Result<Vec<T>, sqlx::Error>;
    async fn insert_to_db_unnest<T>(&self, sports_player_data: &Vec<Self::SportsType>) -> sqlx::Result<()>;

    async fn insert_to_db_querybuilder<T>(&self, sports_player_data: &Vec<serde_json::Value>) -> sqlx::Result<()>;

    async fn query_db_year<T>(&self, table : &str, year: i32) -> Result<Vec<T>, sqlx::Error> where T: DBStructSupport + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin;


    async fn update_db(&self) -> sqlx::Result<()>;
    async fn delete_db(&self) -> sqlx::Result<()>;

}

// 2. The Factory Extension Trait

// 3. Blanket Implementation: The logic is now written ONCE for ALL types



pub fn convert_json_tostring<T>(player: T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    // Convert the struct to a JSON Value
    let value: Value = serde_json::to_value(&player)?;

    if let Some(object) = value.as_object() {
        // Extract keys and join them as a comma-separated string
        let keys_vec: Vec<&str> = object.keys().map(|s| s.as_str()).collect();
        Ok(keys_vec.join(", "))
    } else {
        eprintln!("Error: The provided JSON value is not an object.");
        Ok(String::new())
    }
}



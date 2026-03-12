pub mod nba;

use dioxus::prelude::Signal;
use serde::Serialize;
use serde_json::Value;

pub fn convert_json_tostringvec<T>(player: T) -> Result<Vec<String>, serde_json::Error>
where
    T: Serialize,
{
    // Convert the struct to a JSON Value
    let value: Value = serde_json::to_value(&player)?;
    if let Some(object) = value.as_object() {
        let vec : Vec<String> = object.keys().map(|a| {a.to_string()}).collect();
        Ok(vec)
    } else{
        Ok(vec![])
    }
}

pub fn convert_struct_to_json<T: Serialize>(array: &[T]) -> Result<Vec<serde_json::Value>, serde_json::Error> {
    array.iter().map(serde_json::to_value).collect()
}


pub trait DBStructSupport
{
    const TABLE_NAME: &'static str;
    const SELECT_COLUMNS: &'static str;

    /// Default full SELECT query
    fn select_query_year() -> String {
        format!("SELECT {} FROM {} WHERE start_year = $1", Self::SELECT_COLUMNS, Self::TABLE_NAME)
    }
    fn sort_vector(&self, list: & mut Vec<Self>, column: String, descending: bool) where Self: Sized;
}
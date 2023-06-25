use canyon_sql::macros::*;
use serde::Serialize;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct Stream {
    league_id: Option<i64>,
    provider: String,
    parameter: String,
    locale: String,
    english_name: String,
}

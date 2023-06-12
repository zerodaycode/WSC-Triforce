use canyon_sql::{date_time::NaiveDate, macros::*};
use serde::Serialize;

use super::leagues::League;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct Tournament {
    #[primary_key]
    id: i32,
    ext_id: i64,
    slug: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    #[foreign_key(table = "league", column = "id")]
    league: i32,
}

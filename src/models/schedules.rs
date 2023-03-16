use serde::Serialize;
use canyon_sql::{macros::*, date_time::NaiveDateTime};

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct Schedule {
    #[primary_key]
    id: i32,
    start_time: Option<NaiveDateTime>,
    state: String,
    event_type: String,
    blockname: Option<String>,
    league_id: Option<i64>,
    match_id: Option<i64>,
    strategy: Option<String>,
    strategy_count: Option<i64>,
    team_left_id: Option<i64>,
    team_left_wins: Option<i64>,
    team_right_id: Option<i64>,
    #[foreign_key(table = "schedule", column = "id")]
    team_right_wins: Option<i64>    
}
use canyon_sql::{date_time::NaiveDateTime, macros::*};
use serde::Serialize;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize, Fields)]
#[canyon_entity]
pub struct TeamSchedule {
    #[primary_key]
    id: i32,
    start_time: Option<NaiveDateTime>,
    state: String,
    event_type: String,
    blockname: Option<String>,
    league_name: Option<String>,
    match_id: Option<i64>,
    strategy: Option<String>,
    strategy_count: Option<i64>,
    team_left_id: Option<i64>,
    team_left_wins: Option<i64>,
    team_right_id: Option<i64>,
    team_right_wins: Option<i64>,
    team_left_img_url: Option<String>,
    team_left_name: Option<String>,
    team_right_img_url: Option<String>,
    team_right_name: Option<String>,
}

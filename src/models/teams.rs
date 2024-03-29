use canyon_sql::macros::*;
use serde::Serialize;

#[derive(Debug, Clone, Fields, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct Team {
    #[primary_key]
    id: i32,
    ext_id: i64,
    name: String,
    slug: String,
    code: String,
    image_url: String,
    alt_image_url: Option<String>,
    bg_image_url: Option<String>,
    // home_league: i32
}

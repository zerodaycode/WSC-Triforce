use serde::{Serialize};
use canyon_sql::*;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_macros::canyon_entity]
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
    home_league: Option<i32>
}
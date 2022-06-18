use serde::{Serialize};
use canyon_sql::*;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, ForeignKeyable, Serialize)]
#[canyon_macros::canyon_entity]
pub struct Team {
    id: i32,
    ext_id: i64,
    name: String,
    slug: String,
    code: String,
    image_url: String,
    alt_image_url: String,
    bg_image_url: String,
    home_league: i32
}
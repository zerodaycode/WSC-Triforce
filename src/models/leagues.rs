use canyon_sql::macros::*;
use serde::Serialize;

#[derive(Debug, Clone, CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct League {
    #[primary_key]
    id: i32,
    ext_id: i64,
    slug: String,
    name: String,
    region: String,
    image_url: String,
}

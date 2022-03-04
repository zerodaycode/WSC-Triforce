use serde::{Serialize};
use canyon_sql::*;

#[derive(Clone, Debug, CanyonCRUD, CanyonMapper, Serialize)]
#[canyon_managed]
pub struct Team {
    id: i32,
    name: String,
    slug: String,
    code: String,
    image: String
}
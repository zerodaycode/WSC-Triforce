use crate::utils::triforce_catalog::TriforceCatalog;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SearchBarData {
    pub id: i32,
    pub kind: TriforceCatalog,
    pub entity_name: String,
    pub entity_image_url: String,
    pub entity_alt_data: String,
    pub player_role: Option<String>,
}

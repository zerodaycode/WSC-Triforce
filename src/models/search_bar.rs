use serde::Serialize;
use crate::utils::triforce_catalog::TriforceCatalog;

#[derive(Default, Serialize)]
pub struct SearchBarData {
    pub kind: TriforceCatalog,
    pub entity_name: String,
    pub entity_image_url: String,
    pub entity_alt_data: String,
    pub player_role: Option<String>,
}
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SearchBarData {
    pub entity_name: String,
    pub entity_image_url: String,
    pub entity_alt_data: String,
}
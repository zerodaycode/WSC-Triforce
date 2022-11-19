use std::default;

use serde::Serialize;

/// TODO Docs
#[derive(Debug, Default, Serialize)]
pub enum TriforceCatalog {
    #[default]
    League,
    Tournament,
    Team,
    Player
}

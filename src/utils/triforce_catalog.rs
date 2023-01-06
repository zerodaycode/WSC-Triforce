use serde::Serialize;

/// TODO Docs
#[derive(Debug, Default, Serialize)]
#[allow(unused)]
pub enum TriforceCatalog {
    #[default] League,
    Tournament,
    Team,
    Player
}

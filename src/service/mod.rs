use crate::{utils::constants::lolesports, models::leagues::League};

/// Contains the operations against the `LolEsports` API to
/// fetch the content via REST request that `Triforce` needs
/// to pull, parse, handle and store.
#[derive(Debug)]
pub struct DataPull {

}

impl DataPull {

    pub fn get_leagues() {
        let leagues_url = format!("{}/getLeagues?hl=en-US", lolesports::base_url);
        let response = reqwest::get(&leagues_url).await?;

        let leagues= response.json::<Vec<League>>().await?;
    }
}
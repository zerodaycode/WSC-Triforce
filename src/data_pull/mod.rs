///! Flow of requests for the LoLEsport API and data manipulation
/// For more information about the API (endpoints, parameters, etc) visit https://vickz84259.github.io/lolesports-api-docs/#tag/match-details
/// 
/// First request -> Ask for leagues
/// 
/// Second request -> Ask for tournaments
/// This request must be done by league ID, so we can identify the tournaments for each league
/// 
/// Third request -> Ask for teams
/// 
/// Fourth request -> Ask for players
/// 
/// It is important to note that both teams and players are obtained from the same endpoint
/// 
/// Fifth request -> Ask for schedule
/// 
/// We will insert data with the upsert method, and deleting deprecated data if needed
/// 

pub mod serde_models {
    use canyon_sql::db_clients::tiberius::time::chrono;
    use ::serde::Deserialize;

    #[derive(Deserialize)]
    pub struct League {
        id: i64,
        slug: String,
        name: String,
        region: String,
        image: String,
    }

    #[derive(Deserialize)]
    pub struct Tournament {
        id: i64,
        slug: String,
        #[serde(alias = "startDate")]
        start_date: chrono::NaiveDate,
        #[serde(alias = "endDate")]
        end_date: chrono::NaiveDate,
    }

    #[derive(Deserialize)]
    pub struct Player {
        id: i64,
        #[serde(alias = "firstName")]
        first_name: String,
        #[serde(alias = "lastName")]
        last_name: String,
        #[serde(alias = "summonerName")]
        summoner_name: String,
        image: Option<String>,
        role: String
    }

    #[derive(Deserialize)]
    pub struct Team {
        id: i64,
        name: String,
        slug: String,
        code: String,
        image: String,
        #[serde(alias = "alternativeImage")]
        alternative_image: Option<String>,
        #[serde(alias = "backgroundImage")]
        background_image: Option<String>,
        status: String,
        players: Vec<Player>,
        #[serde(alias = "homeLeague")]
        home_league: League
    }

    #[derive(Deserialize)]
    pub struct Event {
        #[serde(alias = "startTime")]
        start_time: chrono::NaiveDateTime,
        state: String,
        r#type: String,
        #[serde(alias = "blockName")]
        block_name: String,
        league: League,
        teams: Vec<TeamEvent>,
        strategy: Strategy
    }

    #[derive(Deserialize)]
    pub struct TeamEvent {
        team: Team,
        result: MatchTeamResult
    }

    #[derive(Deserialize)]
    pub struct MatchTeamResult {
        outcome: String,
        #[serde(alias = "gameWins")]
        game_wins: String
    }

    #[derive(Deserialize)]
    pub struct Strategy {
        r#type: String,
        count: i8
    }

}

struct LolesportsData {
    leagues: Vec<serde_models::League>,
    tournaments: Vec<serde_models::Tournament>,
    teams: Vec<serde_models::Team>,
    schedule: Vec<serde_models::Event>,
    // ...
}


/// Autonomous process triggered every (X config data [replace this])
/// that queries the lolesports API to fetch data and sync the received
/// data with the one stored in our PostgreSQL
fn scheduled_data_pull_process() {

}

/// Manages the operations needed to query lolespors
fn retrieve_lolesports_data() {
    // Stack of function calls to actually retrieve the data
}

/// Responsable for retrieving the [`crate::`]
fn get_lolesport_leagues() {

}


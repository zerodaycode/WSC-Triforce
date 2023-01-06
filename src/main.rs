extern crate rocket;

mod models;
mod utils;

use rocket::get;
use rocket::http::Status;
use rocket::response::status;

use canyon_sql::crud::CrudOperations;

use models::{
    leagues::League,
    tournaments::Tournament,
    teams::Team,
    players::Player,
    search_bar::SearchBarData
};

use rocket::serde::json::Json;
use utils::triforce_catalog::TriforceCatalog;

#[get("/leagues")]
async fn leagues() -> status::Custom<Json<Vec<League>>> {
    let all_leagues: Result<Vec<League>, _> = League::find_all().await;
    match all_leagues {
        Ok(leagues) => status::Custom(Status::Accepted, Json(leagues)),
        Err(e) => {
            eprintln!("Error on leagues: {:?}", e);
            status::Custom(Status::InternalServerError, Json(vec![]))
        }
    }
}

#[get("/tournaments")]
async fn tournaments() -> status::Custom<Json<Vec<Tournament>>> {
    let all_tournaments: Vec<Tournament> = Tournament::find_all_unchecked().await;
    status::Custom(Status::Accepted, Json(all_tournaments))
}

#[get("/teams")]
async fn teams() -> status::Custom<Json<Vec<Team>>> {
    let all_teams: Vec<Team> = Team::find_all_unchecked().await;
    status::Custom(Status::Accepted, Json(all_teams))
}


#[get("/players")]
async fn players() -> status::Custom<Json<Vec<Player>>> {
    let all_players: Vec<Player> = Player::find_all_unchecked().await;
    status::Custom(Status::Accepted, Json(all_players))
}

#[get("/search-bar-data")]
async fn search_bar_data() -> status::Custom<Json<Vec<SearchBarData>>> {
    let all_teams: Vec<Team> = Team::find_all_unchecked().await;
    let mut search_bar_entities: Vec<SearchBarData> = Vec::new();

    all_teams.into_iter().for_each(|team|
        search_bar_entities.push(SearchBarData {
            kind: TriforceCatalog::Team,
            entity_name: team.name,
            entity_image_url: team.image_url,
            entity_alt_data: team.slug,
        })
    );
    status::Custom(Status::Accepted, Json(search_bar_entities))
}

#[canyon_sql::main]
fn main() {
    rocket::build()
        .mount(
            "/api", 
            rocket::routes![
                leagues,
                tournaments,
                teams,
                players,
                search_bar_data
            ]
        ).launch()
        .await
        .ok(); // TODO Tremendous error handling instead .ok()
}
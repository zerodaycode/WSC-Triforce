extern crate rocket;

use rocket::get;
use rocket::http::Status;
use rocket::response::status;

use canyon_sql::*;
mod team;
mod search_bar;
use search_bar::SearchBarData;
use team::Team;

use rocket::serde::json::Json;


#[get("/")]
async fn teams() -> status::Custom<Json<Vec<Team>>> {
    let all_teams: Vec<Team> = Team::find_all().await;
    status::Custom(Status::Accepted, Json(all_teams))
}

#[get("/search-bar-data")]
async fn search_bar_data() -> status::Custom<Json<Vec<SearchBarData>>> {
    let all_teams: Vec<Team> = Team::find_all().await;
    let mut search_bar_entities: Vec<SearchBarData> = Vec::new();

    all_teams.into_iter().for_each(|team|
        search_bar_entities.push(SearchBarData {
            entity_name: team.name,
            entity_image_url: team.image_url,
            entity_alt_data: team.slug,
        })
    );
    status::Custom(Status::Accepted, Json(search_bar_entities))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![teams])
        .mount("/api", rocket::routes![search_bar_data])
}
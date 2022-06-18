use rocket::http::Status;
use rocket::response::{content, status};

use canyon_sql::*;
mod team;
use team::Team;

use rocket::serde::json::Json;


#[macro_use] extern crate rocket;

#[get("/")]
async fn json() -> Json<String> {
    let all_teams: Vec<Team> = Team::find_all().await;
    let json = serde_json::to_string(&all_teams).unwrap();
    println!("Result: {:?}", json);
    Json(json)
}

#[launch]
#[canyon]
fn rocket() -> _ {
    rocket::build().mount("/", routes![json])
}

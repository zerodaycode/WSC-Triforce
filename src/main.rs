use rocket::http::Status;
use rocket::response::{content, status};

use canyon_sql::*;
mod team;
use team::Team;


#[macro_use] extern crate rocket;

#[get("/")]
async fn json() -> status::Custom<content::Json<String>> {
    let all_teams: Vec<Team> = Team::find_all().await;
    let json = serde_json::to_string(&all_teams).unwrap();
    println!("Result: {:?}", json);
    status::Custom(Status::Accepted, content::Json(json))
}

#[launch]
#[canyon]
fn rocket() -> _ {
    rocket::build().mount("/", routes![json])
}

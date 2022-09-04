extern crate rocket;

use rocket::get;
use rocket::http::Status;
use rocket::response::status;

use canyon_sql::*;
mod team;
use team::Team;

use rocket::serde::json::Json;


#[get("/")]
async fn teams() -> status::Custom<Json<Vec<Team>>> {
    let all_teams: Vec<Team> = Team::find_all().await;
    status::Custom(Status::Accepted, Json(all_teams))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![teams])
}
extern crate rocket;

mod api;
mod models;
mod utils;

use api::controllers::lolesports;

#[canyon_sql::main]
fn main() {
    rocket::build()
        .mount("/api", lolesports::routes())
        .launch()
        .await
        .ok();
}

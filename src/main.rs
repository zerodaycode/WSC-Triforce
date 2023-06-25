extern crate rocket;

mod models;
mod utils;

use rocket::get;
use rocket::http::Status;
use rocket::response::status;

use canyon_sql::{
    crud::{CrudOperations, Transaction},
    query::{operators::Comp, ops::QueryBuilder},
};

use models::{
    leagues::League, players::*, search_bar::SearchBarData, stream::Stream, teams::*,
    tournaments::Tournament, ts::TeamSchedule,
};

use rocket::serde::json::Json;
use utils::triforce_catalog::TriforceCatalog;

use crate::models::league_with_streams::LeagueWithStreams;

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

#[get("/leagues-with-streams")]
async fn leagues_with_streams() -> status::Custom<Json<Vec<LeagueWithStreams>>> {
    let all_leagues: Result<Vec<League>, _> = League::find_all().await;

    let query = format!(
        "SELECT
        s.league_id,
            s.league_id,
            st.provider,
            st.parameter,
            st.locale,
            st.english_name
        FROM
            public.stream st
        JOIN
            public.schedule s ON st.event_id = s.id  AND s.state = 'inProgress'"
    );

    let streams = Stream::query(query, [], "")
        .await
        .map(|r| r.into_results::<Stream>());

    match (all_leagues, streams) {
        (Ok(leagues), Ok(current_streams)) => {
            let mut leagues_with_streams: Vec<LeagueWithStreams> = Vec::new();

            for league in leagues {
                let streams_for_league: Vec<Stream> = current_streams
                    .iter()
                    .filter(|stream| stream.league_id == Some(league.id.into()))
                    .cloned()
                    .collect();
                let league_with_streams =
                    LeagueWithStreams::from_league_and_streams(league, streams_for_league);

                leagues_with_streams.push(league_with_streams);
            }
            status::Custom(Status::Accepted, Json(leagues_with_streams))
        }
        // TODO Implement Error Control
        _ => status::Custom(Status::InternalServerError, Json(vec![])),
    }
}

#[get("/tournaments")]
async fn tournaments() -> status::Custom<Json<Vec<Tournament>>> {
    let all_tournaments: Vec<Tournament> = Tournament::find_all_unchecked().await;
    status::Custom(Status::Accepted, Json(all_tournaments))
}

#[get("/preview-incoming-events")]
async fn preview_incoming_events() -> status::Custom<Json<Vec<TeamSchedule>>> {
    let query = format!(
        "SELECT s.id, s.start_time, s.state, s.event_type, s.blockname, s.match_id, s.strategy, s.strategy_count,
        s.team_left_id, s.team_left_wins, s.team_right_id, s.team_right_wins,
        tl.code AS team_left_name,
        tr.code AS team_right_name,
        tl.image_url AS team_left_img_url,
        tr.image_url AS team_right_img_url,
        l.\"name\" AS league_name
        FROM schedule s
            JOIN team tl ON s.team_left_id = tl.id
            JOIN team tr ON s.team_right_id = tr.id
            JOIN league l ON s.league_id = l.id
        WHERE s.state <> 'completed'
            AND s.event_type = 'match'
            AND NOT (tl.slug = 'tbd' AND tr.slug = 'tbd')
        ORDER BY s.start_time ASC
        FETCH FIRST 30 ROWS ONLY"
    );

    let schedules = TeamSchedule::query(query, [], "")
        .await
        .map(|r| r.into_results::<TeamSchedule>());

    match schedules {
        Ok(v) => status::Custom(Status::Accepted, Json(v)),
        Err(e) => {
            eprintln!("{e}");
            status::Custom(Status::InternalServerError, Json(Vec::new())) // TODO Replace the empty json
        }
    }
}

#[get("/team/<team_id>/schedule")]
async fn find_team_schedule(team_id: i64) -> status::Custom<Json<Vec<TeamSchedule>>> {
    let query = format!(
        "SELECT s.id, s.start_time, s.state, s.event_type, s.blockname, s.match_id, s.strategy, s.strategy_count,
            s.team_left_id, s.team_left_wins, s.team_right_id, s.team_right_wins,
            tl.name AS team_left_name,
            tr.name AS team_right_name,
            tl.image_url AS team_left_img_url,
            tr.image_url AS team_right_img_url,
            l.\"name\" AS league_name
        FROM schedule s
            JOIN team tl ON s.team_left_id = tl.id
            JOIN team tr ON s.team_right_id = tr.id
            JOIN league l ON s.league_id = l.id
        WHERE s.team_left_id = {team_id} OR s.team_right_id = {team_id}
        ORDER BY s.start_time DESC"
    );

    let schedules = TeamSchedule::query(query, [], "")
        .await
        .map(|r| r.into_results::<TeamSchedule>());

    match schedules {
        Ok(v) => status::Custom(Status::Accepted, Json(v)),
        Err(e) => {
            eprintln!("{e}");
            status::Custom(Status::InternalServerError, Json(Vec::new())) // TODO Replace the empty json
        }
    }
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

#[get("/search-bar-data/<query>")]
async fn search_bar_data(query: &str) -> status::Custom<Json<Vec<SearchBarData>>> {
    let mut search_bar_entities: Vec<SearchBarData> = Vec::new();


    let query_teams = format!(
        "SELECT * FROM team t
        WHERE t.\"name\" ILIKE '%{query}%'
        OR t.slug ILIKE '%{query}%'
        OR t.code ILIKE '%{query}%'
        ORDER BY t.id DESC");

    
    let query_players = format!(
        "SELECT * FROM player p
        WHERE p.first_name ILIKE '%{query}%' 
        OR p.last_name ILIKE '%{query}%'
        OR  p.summoner_name  ILIKE '%{query}%'");


        let all_teams = Team::query(query_teams, [], "")
        .await
        .map(|r| r.into_results::<Team>());

        let all_players = Player::query(query_players, [], "")
        .await
        .map(|r| r.into_results::<Player>());

    if let Ok(teams) = all_teams {
        teams.into_iter().for_each(|team| {
            search_bar_entities.push(SearchBarData {
                id: team.id,
                kind: TriforceCatalog::Team,
                entity_name: team.name,
                entity_image_url: team.image_url,
                entity_alt_data: team.slug,
                player_role: None,
            })
        });
    } // TODO Else clause matching an Err kind

    if let Ok(players) = all_players {
        players.into_iter().for_each(|player| {
            search_bar_entities.push(SearchBarData {
                id: player.id,
                kind: TriforceCatalog::Player,
                entity_name: player.summoner_name,
                entity_image_url: player.image_url.unwrap_or_default(),
                entity_alt_data: format!("{} {}", player.first_name, player.last_name),
                player_role: Some(player.role),
            })
        });
    } // TODO Else clause matching an Err kind

    status::Custom(Status::Accepted, Json(search_bar_entities))
}

#[canyon_sql::main]
fn main() {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![
                leagues,
                leagues_with_streams,
                tournaments,
                preview_incoming_events,
                find_team_schedule,
                teams,
                players,
                search_bar_data
            ],
        )
        .launch()
        .await
        .ok(); // TODO Tremendous error handling instead .ok()
}

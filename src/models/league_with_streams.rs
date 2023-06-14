use serde::Serialize;

use super::{stream::Stream, leagues::League};

#[derive(Default, Serialize)]
pub struct LeagueWithStreams {
    id: i32,
    ext_id: i64,
    slug: String,
    name: String,
    region: String,
    image_url: String,
    pub streams: Vec<Stream>
}

impl LeagueWithStreams {
    pub fn from_league_and_streams(league: League, streams: Vec<Stream>) -> Self {
        LeagueWithStreams {
            id: league.id,
            ext_id: league.ext_id,
            slug: league.slug,
            name: league.name,
            region: league.region,
            image_url: league.image_url,
            streams,
        }
    }
}
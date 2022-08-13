use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

const CURRENT: &'static str = "current";
const NEXT: &'static str = "next";
const BATTLE_ROYAL: &'static str = "battle_royal";
const BATTLE_ROYAL_RANKED: &'static str = "ranked";
const ARENA: &'static str = "arenas";
const ARENA_RANKED: &'static str = "arenasRanked";
const EVENT: &'static str = "ltm";

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    #[serde(rename = "map")]
    name: String,
    start: i64,
    end: i64,
    code: String,
    asset: String,
}

impl Map {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn start(&self) -> i64 {
        self.start
    }
    fn end(&self) -> i64 {
        self.end
    }
    fn start_as_date(&self) -> DateTime<Utc> {
        DateTime::from_utc(NaiveDateTime::from_timestamp(self.start, 0), Utc)
    }
    fn end_as_date(&self) -> DateTime<Utc> {
        DateTime::from_utc(NaiveDateTime::from_timestamp(self.end, 0), Utc)
    }
    fn asset(&self) -> String {
        self.asset.clone()
    }
    fn asset_as_url(&self) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Url::parse(&self.asset)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapRotation {
    maps: HashMap<String, Map>,
}

impl MapRotation {
    fn current<'a>(&'a self) -> Option<&'a Map> {
        self.maps.get(CURRENT)
    }
    fn next<'a>(&'a self) -> Option<&'a Map> {
        self.maps.get(NEXT)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapRotations {
    rotations: HashMap<String, MapRotation>,
}

impl MapRotations {
    fn battle_royal<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(BATTLE_ROYAL)
    }
    fn battle_royal_ranked<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(BATTLE_ROYAL_RANKED)
    }
    fn arena<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(ARENA)
    }
    fn arena_ranked<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(ARENA_RANKED)
    }
    fn event<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(EVENT)
    }
}

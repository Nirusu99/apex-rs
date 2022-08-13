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
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn start(&self) -> i64 {
        self.start
    }
    pub fn end(&self) -> i64 {
        self.end
    }
    pub fn start_as_date(&self) -> DateTime<Utc> {
        DateTime::from_utc(NaiveDateTime::from_timestamp(self.start, 0), Utc)
    }
    pub fn end_as_date(&self) -> DateTime<Utc> {
        DateTime::from_utc(NaiveDateTime::from_timestamp(self.end, 0), Utc)
    }
    pub fn asset(&self) -> String {
        self.asset.clone()
    }
    pub fn asset_as_url(&self) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Url::parse(&self.asset)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapRotation {
    maps: HashMap<String, Map>,
}

impl MapRotation {
    pub fn current<'a>(&'a self) -> Option<&'a Map> {
        self.maps.get(CURRENT)
    }
    pub fn next<'a>(&'a self) -> Option<&'a Map> {
        self.maps.get(NEXT)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapRotations {
    rotations: HashMap<String, MapRotation>,
}

impl MapRotations {
    pub fn battle_royal<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(BATTLE_ROYAL)
    }
    pub fn battle_royal_ranked<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(BATTLE_ROYAL_RANKED)
    }
    pub fn arena<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(ARENA)
    }
    pub fn arena_ranked<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(ARENA_RANKED)
    }
    pub fn event<'a>(&'a self) -> Option<&'a MapRotation> {
        self.rotations.get(EVENT)
    }
}

use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

const CURRENT: &'static str = "current";
const NEXT: &'static str = "next";
const BATTLE_ROYAL: &'static str = "battle_royale";
const BATTLE_ROYAL_RANKED: &'static str = "ranked";
const ARENA: &'static str = "arenas";
const ARENA_RANKED: &'static str = "arenasRanked";
const EVENT: &'static str = "ltm";

fn convert_stamp_to_utc(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
}

fn ok_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: serde::Deserialize<'de> + Default,
    D: serde::Deserializer<'de>,
{
    let v: serde_json::Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Map {
    #[serde(rename = "map")]
    name: String,
    start: i64,
    end: i64,
    code: String,
    // WHY?! when you get the rotation with `version=1` the next map doesn't have
    // an asset, when you do it with `version=2` it does?????????
    // thx api creators
    asset: Option<String>,
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
        convert_stamp_to_utc(self.start)
    }
    pub fn end_as_date(&self) -> DateTime<Utc> {
        convert_stamp_to_utc(self.end)
    }
    pub fn asset(&self) -> Option<String> {
        self.asset.clone()
    }
    pub fn asset_as_url(&self) -> Result<Option<Url>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.asset {
            Some(url) => Ok(Some(Url::parse(url)?)),
            None => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MapRotation {
    #[serde(flatten)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum BundleType {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "permanent")]
    Permanent,
    Unknown,
}

impl Default for BundleType {
    fn default() -> BundleType {
        BundleType::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Bundle {
    bundle: String,
    #[serde(default)]
    start: i64,
    #[serde(default)]
    end: i64,
    #[serde(rename = "bundleType", default, deserialize_with = "ok_or_default")]
    bundle_type: BundleType,
    #[serde(rename = "bundleContent")]
    items: Vec<BundleItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BundleItem {
    #[serde(rename = "item")]
    name: String,
    cost: u16,
    #[serde(rename = "itemType")]
    item_type: ItemType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemType {
    name: String,
    #[serde(default, deserialize_with = "ok_or_default")]
    rarity: Rarity,
    asset: String,
    #[serde(rename = "rarityHex")]
    color: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Rarity {
    Unknown,
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Default for Rarity {
    fn default() -> Rarity {
        Rarity::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Bundles {
    bundles: Vec<Bundle>,
}

impl Bundles {
    pub fn daily_bundles(&self) -> Vec<&Bundle> {
        self.bundles
            .iter()
            .filter(|bundle| bundle.bundle_type == BundleType::Daily)
            .collect()
    }
    pub fn weekly_bundles(&self) -> Vec<&Bundle> {
        self.bundles
            .iter()
            .filter(|bundle| bundle.bundle_type == BundleType::Weekly)
            .collect()
    }
    pub fn permanent_bundles(&self) -> Vec<&Bundle> {
        self.bundles
            .iter()
            .filter(|bundle| bundle.bundle_type == BundleType::Permanent)
            .collect()
    }
}

impl<'a> Bundle {
    pub fn bundle(&self) -> &str {
        &self.bundle
    }
    pub fn start(&self) -> i64 {
        self.start
    }
    pub fn end(&self) -> i64 {
        self.end
    }
    pub fn start_as_date(&self) -> DateTime<Utc> {
        convert_stamp_to_utc(self.start)
    }
    pub fn end_as_date(&self) -> DateTime<Utc> {
        convert_stamp_to_utc(self.end)
    }
    pub fn items(&'a self) -> Vec<&'a BundleItem> {
        self.items.iter().collect()
    }
    pub fn bundle_type(&'a self) -> &'a BundleType {
        &self.bundle_type
    }
}

impl<'a> BundleItem {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn item_type(&'a self) -> &'a ItemType {
        &self.item_type
    }

    pub fn cost(&self) -> u16 {
        self.cost
    }
}

impl<'a> ItemType {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rarity(&'a self) -> &'a Rarity {
        &self.rarity
    }

    pub fn asset(&self) -> &str {
        &self.asset
    }
    pub fn asset_as_url(&self) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Url::parse(&self.asset)?)
    }
    pub fn color_hex(&self) -> &str {
        &self.color
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Bundles, MapRotations, Rarity};

    #[test]
    fn parse_example() -> Result<(), Box<dyn std::error::Error>> {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("resources/test/example_request_body");
        let maps = std::fs::read_to_string(&format!("{}", root.display()))?;
        root.pop();
        root.push("example_request_body_crafter");
        let crafter = std::fs::read_to_string(&format!("{}", root.display()))?;
        root.pop();
        root.push("example_request_body_missing_fields");
        let crafter_mis = std::fs::read_to_string(&format!("{}", root.display()))?;
        let _: MapRotations = serde_json::from_str(&maps)?;
        let _: Bundles = serde_json::from_str(&crafter)?;
        let _: Bundles = serde_json::from_str(&crafter_mis)?;
        Ok(())
    }
    #[test]
    fn test_rarity_order() {
        assert!(Rarity::Legendary > Rarity::Common);
        assert!(Rarity::Legendary > Rarity::Rare);
        assert!(Rarity::Legendary > Rarity::Epic);
        assert!(Rarity::Legendary > Rarity::Unknown);
        assert!(Rarity::Epic > Rarity::Common);
        assert!(Rarity::Epic > Rarity::Rare);
        assert!(Rarity::Epic > Rarity::Unknown);
        assert!(Rarity::Rare > Rarity::Common);
        assert!(Rarity::Rare > Rarity::Unknown);
        assert!(Rarity::Common > Rarity::Unknown);
    }
}

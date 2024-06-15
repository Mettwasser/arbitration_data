use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

pub type ExportRegions<'a> = HashMap<&'a str, SolNodeMapValue>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolNodeMapValue {
    #[serde(rename = "name")]
    pub(crate) node: String, // needs map

    #[serde(rename = "systemName")]
    pub(crate) planet: String, // needs map

    #[serde(rename = "missionName")]
    pub(crate) mission_type: String, // needs map

    #[serde(rename = "factionIndex")]
    pub(crate) faction: Faction,

    pub(crate) dark_sector_data: Option<DarkSectorData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DarkSectorData {
    resource_bonus: f64,

    xp_bonus: f64,

    weapon_xp_bonus_for: WeaponXpBonusFor,

    weapon_xp_bonus_val: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WeaponXpBonusFor {
    Melee,
    Pistols,
    Rifles,
    Shotguns,
}

#[derive(Serialize, Deserialize_repr, Debug, Clone, derive_more::Display)]
#[repr(u8)]
pub enum Faction {
    Grineer,
    Corpus,
    Infested,
    Orokin,
    Sentient = 5,
    Murmur = 7,
}

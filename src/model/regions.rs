use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;

use super::dict::LanguageDict;

pub type SolNodeMap<'a> = HashMap<&'a str, SolNodeMapValue>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolNodeMapValue {
    name: String, // needs map

    system_index: i64,

    system_name: String, // needs map

    node_type: i64,

    mastery_req: i64,

    mission_index: i64,

    mission_name: String, // needs map

    faction_index: i64,

    faction_name: String, // needs

    min_enemy_level: i64,

    max_enemy_level: i64,

    dark_sector_data: Option<DarkSectorData>,

    mastery_exp: Option<i64>,

    secondary_faction_index: Option<i64>,

    secondary_faction_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DarkSectorData {
    resource_bonus: f64,

    xp_bonus: f64,

    weapon_xp_bonus_for: WeaponXpBonusFor,

    weapon_xp_bonus_val: f64,
}

#[derive(Serialize, Deserialize)]
pub enum WeaponXpBonusFor {
    Melee,

    Pistols,

    Rifles,

    Shotguns,
}

fn make_to_region_dict<'a>(s: &'a str, dict: LanguageDict<'a>) -> Result<SolNodeMap<'a>> {
    let mut sol_node_map: SolNodeMap = serde_json::from_str(s)?;
    let final_map: SolNodeMap = HashMap::new();

    for (key, mut value) in sol_node_map.iter_mut() {
        if let Some(&actual_name) = dict.get(value.name.as_ref()) {
            sol_node_map.;
        }
    }
    Ok(sol_node_map)
}

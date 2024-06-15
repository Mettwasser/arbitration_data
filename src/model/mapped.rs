use std::{
    collections::BTreeMap,
    ops::{Div, Rem},
};

use chrono::{Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::{ParseError, ToResult},
    DateTime,
};

use heck::ToTitleCase;

use super::{
    dict::LanguageDict,
    regions::{DarkSectorData, ExportRegions, Faction},
};

pub const MAP_RANKING: phf::Map<&str, Tier> = phf::phf_map!(
    "Cinxia" => Tier::S,
    "Casta" => Tier::S,
    "Seimeni" => Tier::S,
    "Sechura" => Tier::A,
    "Hydron" => Tier::A,
    "Odin" => Tier::A,
    "Helene" => Tier::A,
    "Tessera" => Tier::B,
    "Ose" => Tier::B,
    "Hyf" => Tier::B,
    "Outer Terminus" => Tier::B,
    "Larzac" => Tier::C,
    "Sinai" => Tier::C,
    "Sangeru" => Tier::C,
    "Gulliver" => Tier::C,
    "Alator" => Tier::C,
    "Stephano" => Tier::C,
    "Io" => Tier::C,
    "Kala-azar" => Tier::C,
    "Lares" => Tier::C,
    "Lith" => Tier::C,
    "Paimon" => Tier::C,
    "Callisto" => Tier::C,
    "Bellinus" => Tier::C,
    "Cerberus" => Tier::C,
    "Spear" => Tier::C,
    "Umbriel" => Tier::C,
    "Coba" => Tier::D,
    "Kadesh" => Tier::D,
    "Romula" => Tier::D,
    "Rhea" => Tier::D,
    "Berehynia" => Tier::D,
    "Oestrus" => Tier::D,
    "Proteus" => Tier::D,
    "Xini" => Tier::D,
    "Cytherean" => Tier::D,
    "Stöfler" => Tier::D,
    "Taranis" => Tier::D,
    "Mithra" => Tier::D,
    "Gaia" => Tier::D,
    "Caelus" => Tier::D,
    "Akkad" => Tier::D
);

#[derive(Debug, Deserialize)]
struct ArbitrationTimeMappingRow {
    time: i64,
    node: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    F,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrationInfo {
    pub node: String, // needs map

    pub planet: String, // needs map

    pub mission_type: String, // needs map

    pub faction: Faction,

    pub tier: Tier,

    pub dark_sector_data: Option<DarkSectorData>,

    pub activation: DateTime,

    pub expiry: DateTime,

    pub eta: String,
}

fn divmod<T>(number: T, other: T) -> (T, T)
where
    T: Rem<Output = T> + Div<Output = T> + Copy,
{
    (number / other, number % other)
}

pub(crate) fn get_short_format_time_string(dt: DateTime) -> String {
    let now = Utc::now();
    let mut time_in_between = (if now > dt { now - dt } else { dt - now }).num_seconds();

    const TIME_COMPONENTS: [(&str, i64); 5] = [
        ("w", 60 * 60 * 24 * 7),
        ("d", 60 * 60 * 24),
        ("h", 60 * 60),
        ("m", 60),
        ("s", 1),
    ];

    let mut formatted_time = String::new();

    for &(suffix, divisor) in &TIME_COMPONENTS {
        let (div_time, mod_time) = divmod(time_in_between, divisor);
        if div_time > 0 {
            formatted_time.push_str(&format!("{}{} ", div_time, suffix));
            time_in_between = mod_time;
        }
    }
    if now > dt {
        formatted_time.push_str("ago")
    } else {
        formatted_time = format!("in {}", formatted_time)
    }

    formatted_time.trim().to_string()
}

pub(crate) fn make_to_arbi_data(
    arbi_time_node_mapping: csv::Reader<&[u8]>,
    export_regions: ExportRegions,
    language_dict: LanguageDict,
) -> std::result::Result<BTreeMap<i64, ArbitrationInfo>, ParseError> {
    let mut arbi_data = BTreeMap::<i64, ArbitrationInfo>::new();
    let mut reader = arbi_time_node_mapping;
    reader.set_headers(vec!["time", "node"].into());

    for row in reader.deserialize() {
        let row: ArbitrationTimeMappingRow = row.map_err(ParseError::Csv)?;

        let node: String;
        let planet: String;
        let mission_type: String;
        let tier: Tier;
        let activation = Utc.timestamp_opt(row.time, 0).to_result()?;
        let expiry = activation + Duration::hours(1);
        let eta = get_short_format_time_string(activation);

        if let Some(value) = export_regions.get(row.node.as_str()) {
            if let Some(actual_node) = language_dict.get(&value.node) {
                node = actual_node.clone();
            } else {
                node = row.node;
            }

            if let Some(actual_planet) = language_dict.get(&value.planet) {
                planet = actual_planet.clone();
            } else {
                planet = value.planet.clone();
            }

            if let Some(actual_mission_name) = language_dict.get(&value.mission_type) {
                mission_type = actual_mission_name.to_title_case();
            } else {
                mission_type = value.mission_type.clone();
            }

            if let Some(actual_tier) = MAP_RANKING.get(&node) {
                tier = actual_tier.clone();
            } else {
                tier = Tier::F;
            }

            arbi_data.insert(
                row.time,
                ArbitrationInfo {
                    node,
                    planet,
                    mission_type,
                    tier,
                    eta,
                    activation,
                    expiry,
                    faction: value.faction.clone(),
                    dark_sector_data: value.dark_sector_data.clone(),
                },
            );
        }
    }
    Ok(arbi_data)
}

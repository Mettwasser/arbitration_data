use std::{
    collections::{btree_map, BTreeMap, HashMap},
    iter::Filter,
};

use chrono::{Duration, Utc};
use error::{Error, ParseError};
use model::{
    dict::LanguageDict,
    mapped::{make_to_arbi_data, ArbitrationInfo},
    regions::SolNodeMapValue,
};
use time_calc::HourOnly;

pub mod error;
pub mod model;
pub mod time_calc;

pub use model::mapped::Tier;
pub use model::regions::Faction;

pub type Result<T> = std::result::Result<T, Error>;
pub type DateTime = chrono::DateTime<chrono::Utc>;

type IterUpcoming<'a> =
    Filter<btree_map::Iter<'a, i64, ArbitrationInfo>, fn(&(&i64, &ArbitrationInfo)) -> bool>;

pub struct ArbitrationData {
    inner: BTreeMap<i64, ArbitrationInfo>,
    max: i64,
}

impl ArbitrationData {
    pub fn new(
        arbi_time_node_mapping: csv::Reader<&[u8]>,
        export_regions: HashMap<&str, SolNodeMapValue>,
        language_dict: LanguageDict,
    ) -> std::result::Result<Self, ParseError> {
        let inner = make_to_arbi_data(arbi_time_node_mapping, export_regions, language_dict)?;
        let max = inner.keys().max().unwrap_or(&0).to_owned();

        Ok(Self { max, inner })
    }

    // region: INNER
    pub fn inner(&self) -> &BTreeMap<i64, ArbitrationInfo> {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut BTreeMap<i64, ArbitrationInfo> {
        &mut self.inner
    }
    pub fn into_inner(self) -> BTreeMap<i64, ArbitrationInfo> {
        self.inner
    }
    // endregion

    pub fn upcoming(&self) -> Result<&ArbitrationInfo> {
        let next_hour = (Utc::now() + Duration::hours(1)).hour_only()?;

        self.inner
            .get(&next_hour.timestamp())
            .ok_or(Error::DataNotFound)
    }

    pub fn upcoming_by_tier(&self, tier: Tier) -> Result<&ArbitrationInfo> {
        let mut next = Utc::now().hour_only()?.timestamp();

        while self.max > next {
            next += Duration::hours(1).num_seconds();
            let entry = match self.inner.get(&next) {
                Some(arbi_data) => arbi_data,
                None => continue,
            };

            if entry.tier == tier {
                return Ok(entry);
            }
        }

        Err(Error::DataNotFound)
    }

    pub fn iter_upcoming(&self) -> IterUpcoming<'_> {
        self.inner
            .iter()
            .filter(|item| *item.0 > Utc::now().timestamp())
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use csv::Reader;

    use crate::{
        model::{dict::LanguageDict, mapped::Tier, regions::ExportRegions},
        ArbitrationData,
    };

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let language_dict: LanguageDict = serde_json::from_str(include_str!("../dict.en.json"))?;
        let export_regions: ExportRegions =
            serde_json::from_str(include_str!("../ExportRegions.json"))?;

        let data = ArbitrationData::new(
            Reader::from_reader(include_bytes!("../arbys.csv")),
            export_regions,
            language_dict,
        )?;

        println!(
            "{:#?}",
            data.iter_upcoming()
                .find(|(_, info)| info.tier == Tier::S || info.tier == Tier::A)
        );

        Ok(())
    }
}

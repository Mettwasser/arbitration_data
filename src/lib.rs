use std::collections::HashMap;

use chrono::{Duration, Utc};
use error::{Error, InnerError};
use model::{
    dict::LanguageDict,
    mapped::{make_to_arbi_data, ArbitrationInfo, Tier},
    regions::SolNodeMapValue,
};
use time_calc::DateTimeExt;

pub mod error;
pub mod model;
pub mod time_calc;

pub type Result<T> = std::result::Result<T, Error>;
pub type DateTime = chrono::DateTime<chrono::Utc>;

pub struct ArbitrationData {
    inner: HashMap<i64, ArbitrationInfo>,
    max: i64,
}

impl ArbitrationData {
    pub fn new(
        arbi_time_node_mapping: csv::Reader<&[u8]>,
        export_regions: HashMap<&str, SolNodeMapValue>,
        language_dict: LanguageDict,
    ) -> Result<Self> {
        let inner = make_to_arbi_data(arbi_time_node_mapping, export_regions, language_dict)?;
        let max = inner
            .keys()
            .max()
            .ok_or(Error::Initialization(InnerError {
                message: "Empty collection".to_string(),
            }))?
            .to_owned();

        Ok(Self { max, inner })
    }

    // region: INNER
    pub fn inner(&self) -> &HashMap<i64, ArbitrationInfo> {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut HashMap<i64, ArbitrationInfo> {
        &mut self.inner
    }
    pub fn into_inner(self) -> HashMap<i64, ArbitrationInfo> {
        self.inner
    }
    // endregion

    pub fn upcoming(&self) -> Option<&ArbitrationInfo> {
        let next_hour = (Utc::now() + Duration::hours(1)).hour_only()?;

        self.inner.get(&next_hour.timestamp())
    }

    pub fn upcoming_by_tier(&self, tier: Tier) -> Option<&ArbitrationInfo> {
        let mut next = Utc::now().hour_only()?.timestamp();

        while self.max > next {
            next += Duration::hours(1).num_seconds();
            let entry = match self.inner.get(&next) {
                Some(arbi_data) => arbi_data,
                None => continue,
            };

            if entry.tier == tier {
                return Some(entry);
            }
        }

        None
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

        println!("{:#?}", data.upcoming_by_tier(Tier::S));

        Ok(())
    }
}

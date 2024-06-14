use chrono::Timelike;

use crate::{error::Error, Result};

pub trait HourOnly: Sized {
    /// Returns a DateTime with everything but the hour set to 0.
    /// Returns [Error::InvalidatedDateTime] if then DateTime would be invalid because of this.
    fn hour_only(&self) -> Result<Self>;
}

impl<T> HourOnly for T
where
    T: Timelike,
{
    fn hour_only(&self) -> Result<Self> {
        Ok(self
            .with_minute(0)
            .and_then(|dt| dt.with_second(0))
            .and_then(|dt| dt.with_nanosecond(0))
            .ok_or(Error::InvalidatedDateTime)?)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, TimeZone, Utc};

    use crate::time_calc::HourOnly;

    #[test]
    fn test_hour_only() {
        let dt = Utc.timestamp_opt(1718295508, 0).unwrap();
        let next_hour = (dt + Duration::hours(1)).hour_only().unwrap();
        assert_eq!(next_hour.timestamp(), 1718298000)
    }
}

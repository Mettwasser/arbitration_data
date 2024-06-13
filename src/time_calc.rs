use crate::DateTime;
use chrono::Timelike;

pub trait DateTimeExt: Sized {
    /// Returns a DateTime with everything but the hour set to 0.
    /// Returns [None] if then DateTime would be invalid because of this.
    fn hour_only(&self) -> Option<Self>;
}

impl DateTimeExt for DateTime {
    fn hour_only(&self) -> Option<Self> {
        self.with_minute(0)?.with_second(0)?.with_nanosecond(0)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, TimeZone, Utc};

    use crate::time_calc::DateTimeExt;

    #[test]
    fn test_upcoming_hour_timestamp() {
        let dt = Utc.timestamp_opt(1718295508, 0).unwrap();
        let next_hour = (dt + Duration::hours(1)).hour_only().unwrap();
        assert_eq!(next_hour.timestamp(), 1718298000)
    }
}

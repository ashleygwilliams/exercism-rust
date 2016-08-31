extern crate chrono;
use chrono::*;

const GIGASECOND:i64 = 1000000000;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
  DateTime::from_utc(NaiveDateTime::from_timestamp(date.timestamp() + GIGASECOND, 0), UTC)
}

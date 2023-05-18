use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct NaiveDateTimeWrapper(pub NaiveDateTime);

impl NaiveDateTimeWrapper {
    pub fn new(date_time: NaiveDateTime) -> NaiveDateTimeWrapper {
        NaiveDateTimeWrapper(date_time)
    }

    pub fn parse_from_str(date_time: &str, fmt: &str) -> Result<NaiveDateTimeWrapper, ()> {
        let date_time = NaiveDateTime::parse_from_str(date_time, &fmt);

        match date_time {
            Ok(date_time) => Ok(NaiveDateTimeWrapper(date_time)),
            Err(_) => Err(()),
        }
    }
}

impl std::fmt::Display for NaiveDateTimeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", NaiveDateTime::to_string(&self.0))
    }
}

impl Serialize for NaiveDateTimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.0.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

impl Clone for NaiveDateTimeWrapper {
    fn clone(&self) -> Self {
        NaiveDateTimeWrapper(self.0.clone())
    }
}

impl FromStr for NaiveDateTimeWrapper {
    type Err = ();

    fn from_str(input: &str) -> Result<NaiveDateTimeWrapper, Self::Err> {
        let date_time = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S");

        match date_time {
            Ok(date_time) => Ok(NaiveDateTimeWrapper(date_time)),
            Err(_) => Err(()),
        }
    }
}
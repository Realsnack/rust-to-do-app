use std::str::FromStr;

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SupportedPersistence {
    CSV,
    Postgres,
}

impl ToString for SupportedPersistence {
    fn to_string(&self) -> String {
        match self {
            SupportedPersistence::CSV => "CSV".to_string(),
            SupportedPersistence::Postgres => "Postgres".to_string(),
        }
    }
}

impl FromStr for SupportedPersistence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(SupportedPersistence::CSV),
            "postgres" => Ok(SupportedPersistence::Postgres),
            _ => Err(()),
        }
    }
}

use crate::RRType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub name: String,
    pub r#type: Option<RRType>,
    pub r#do: bool, // DNSSEC data
    pub cd: bool,   // disable validation
}

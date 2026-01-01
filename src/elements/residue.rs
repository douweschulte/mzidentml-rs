use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Residue {
    #[serde(rename = "@code")]
    pub code: char,
    #[serde(rename = "@mass")]
    pub mass: f64,
}

impl IsElement for Residue {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cv {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: String,
    #[serde(rename = "@version")]
    pub version: Option<String>, // TODO: Implement sem ver like struct
    #[serde(rename = "@uri")]
    pub uri: String, // TODO: Proper uri check
}

impl IsElement for Cv {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("CV", "id]"));
        }

        if self.full_name.is_empty() {
            return Err(ValidationError::EmptyAttribute("CV", "full_name"));
        }

        if self.uri.is_empty() {
            return Err(ValidationError::EmptyAttribute("CV", "uri"));
        }

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "@unitAccession")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitCvRef")]
    pub unit_cv_ref: Option<String>,
    #[serde(rename = "@unitName")]
    pub unit_name: Option<String>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl IsElement for UserParam {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::EmptyAttribute("UserParam", "name"));
        }
        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    elements::{enzyme_name::EnzymeName, is_element::IsElement, site_regexp::SiteRegexp},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enzyme {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@cTermGain")]
    pub c_term_gain: Option<String>,
    #[serde(rename = "@minDistance")]
    pub min_distance: Option<usize>,
    #[serde(rename = "@missedCleavages")]
    pub missed_cleavages: Option<usize>,
    #[serde(rename = "@nTermGain")]
    pub n_term_gain: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@semiSpecific")]
    pub semi_specific: Option<bool>,
    #[serde(rename = "SiteRegexp")]
    pub site_regexp: Option<SiteRegexp>,
    #[serde(rename = "EnzymeName")]
    pub enzyme_name: Option<EnzymeName>,
}

impl IsElement for Enzyme {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("Enzyme", "id"));
        }

        if let Some(site_regexp) = &self.site_regexp {
            site_regexp.validate(strict)?;
        }

        if let Some(enzyme_name) = &self.enzyme_name {
            enzyme_name.validate(strict)?;
        }

        Ok(())
    }
}

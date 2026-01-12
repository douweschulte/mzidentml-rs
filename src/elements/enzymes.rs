use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, enzyme::Enzyme, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enzymes {
    #[serde(rename = "@independent")]
    pub independent: Option<bool>,

    #[serde(rename = "Enzyme")]
    pub enzyme: Vec<Enzyme>,
}

impl IsElement for Enzymes {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.enzyme.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "Enzymes", "Enzyme",
            ));
        }

        for enz in &self.enzyme {
            enz.validate(version, strict)?;
        }
        Ok(())
    }
}

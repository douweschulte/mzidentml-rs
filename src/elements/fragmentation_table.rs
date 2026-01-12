use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement, measure::Measure},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentationTable {
    #[serde(rename = "Measure")]
    pub measures: Vec<Measure>,
}

impl IsElement for FragmentationTable {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.measures.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "FragmentationTable",
                "Measure",
            ));
        }

        for measure in &self.measures {
            measure.validate(version, strict)?;
        }

        Ok(())
    }
}

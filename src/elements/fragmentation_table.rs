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
    const ELEMENT_TAG: &str = "FragmentationTable";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.measures.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "Measure",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.measures.iter())
    }
}

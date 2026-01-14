use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, ions_type::IonType, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fragmentation {
    #[serde(rename = "IonType")]
    pub ion_types: Vec<IonType>,
}

impl IsElement for Fragmentation {
    const ELEMENT_TAG: &str = "Fragmentation";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.ion_types.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "IonType",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.ion_types.iter())
    }
}

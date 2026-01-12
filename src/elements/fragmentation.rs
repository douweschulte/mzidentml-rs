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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.ion_types.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "Fragmentation",
                "IonType",
            ));
        }

        for ion_type in self.ion_types.iter() {
            ion_type.validate(version, strict)?;
        }

        Ok(())
    }
}

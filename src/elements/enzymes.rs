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
    const ELEMENT_TAG: &str = "Enzymes";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.enzyme.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "Enzyme",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.enzyme.iter())?;
        Ok(())
    }
}

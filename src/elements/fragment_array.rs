use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
    parsing::space_separated_vec_parsing,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentArray {
    #[serde(rename = "@measure_ref")]
    pub measure_ref: String,
    #[serde(rename = "@values", with = "space_separated_vec_parsing")]
    pub values: Vec<f64>,
}

impl IsElement for FragmentArray {
    const ELEMENT_TAG: &str = "FragmentArray";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.measure_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "measure_ref",
            ));
        }

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::{elements::attributes::semver::SemVer, error::ValidationError};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Seq(String);

impl IsElement for Seq {
    const ELEMENT_TAG: &str = "Seq";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        _element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

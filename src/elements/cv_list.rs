use serde::{Deserialize, Serialize};

use crate::{elements::attributes::semver::SemVer, error::ValidationError};

use super::{cv::Cv, is_element::IsElement, is_list::IsList};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "cv")]
    pub cv: Vec<Cv>,
}

impl IsElement for CvList {
    const ELEMENT_TAG: &str = "cvList";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(version, strict, element_path, self.cv.iter())?;
        Ok(())
    }
}

impl IsList<'_, Cv> for CvList {
    fn list_items(&self) -> &[Cv] {
        &self.cv
    }
}

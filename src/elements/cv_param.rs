use serde::{Deserialize, Serialize};

use crate::{elements::attributes::semver::SemVer, error::ValidationError, parsing::cv_id_parsing};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CvParam {
    #[serde(rename = "@cvRef")]
    pub cv_ref: String,
    #[serde(rename = "@accession", with = "cv_id_parsing")]
    pub accession: (String, usize),
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@unitCvRef", skip_serializing_if = "Option::is_none")]
    pub unit_cv_ref: Option<String>,
    #[serde(rename = "@unitAccession", skip_serializing_if = "Option::is_none")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl IsElement for CvParam {
    const ELEMENT_TAG: &str = "cvParam";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.cv_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "cvRef",
            ));
        }
        if self.name.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "name",
            ));
        }

        Ok(())
    }
}

impl PartialEq for CvParam {
    fn eq(&self, other: &Self) -> bool {
        self.accession.0 == other.accession.0 && self.accession.1 == other.accession.1
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BibliographicReference {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@authors")]
    pub authors: Option<String>,
    #[serde(rename = "@doi")]
    pub doi: Option<String>,
    #[serde(rename = "@editor")]
    pub editor: Option<String>,
    #[serde(rename = "@issue")]
    pub issue: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@pages")]
    pub pages: Option<String>,
    #[serde(rename = "@publication")]
    pub publication: Option<String>,
    #[serde(rename = "@publisher")]
    pub publisher: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,
    #[serde(rename = "@volume")]
    pub volume: Option<String>,
    #[serde(rename = "@year")]
    pubyear: Option<usize>,
}

impl IsElement for BibliographicReference {
    const ELEMENT_TAG: &str = "BibliographicReference";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        Ok(())
    }
}

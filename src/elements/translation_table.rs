use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranslationTable {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for TranslationTable {
    const ELEMENT_TAG: &str = "TranslationTable";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(
    TranslationTable,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001410,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001025,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001423,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
    ]
);

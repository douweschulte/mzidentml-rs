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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        self.validate_cv_params(version, strict)
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

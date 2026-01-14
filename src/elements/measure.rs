use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measure {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Measure {
    const ELEMENT_TAG: &str = "Measure";

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

        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(
    Measure,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001226,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001225,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001227,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        }
    ]
);

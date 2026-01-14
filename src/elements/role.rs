use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_param,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "cvParam")]
    pub cv_param: CvParam,
}

impl IsElement for Role {
    const ELEMENT_TAG: &str = "Role";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_param!(
    Role,
    cv_param,
    [CvParamRule {
        cv_name: "MS",
        id: 1001266,
        occurence: CvParamOccurence::MustOnceOrMany,
        supplies_children: true,
    },]
);

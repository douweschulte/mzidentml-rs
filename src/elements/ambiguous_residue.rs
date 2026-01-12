use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbiguousResidue {
    #[serde(rename = "@code")]
    pub code: char,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for AmbiguousResidue {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AmbiguousResidue",
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AmbiguousResidue",
                "userParam",
            ));
        }

        for param in &self.user_params {
            param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    AmbiguousResidue,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001359,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

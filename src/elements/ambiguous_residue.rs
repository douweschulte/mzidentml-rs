use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AmbiguousResidue",
                "cvParam",
            ));
        }

        self.validate_cv_params(strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AmbiguousResidue",
                "userParam",
            ));
        }

        for param in &self.user_params {
            param.validate(strict)?;
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

use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exclude {
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for Exclude {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Exclude", "cvParam"));
        }

        self.validate_cv_params(strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Exclude", "userParam"));
        }

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    Exclude,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001512,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

// TODO: Fully redundant to Exclude, except the error messages. Merge both and create enum or alias?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Include {
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for Include {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Include", "cvParam"));
        }

        self.validate_cv_params(strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Include", "userParam"));
        }

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    Include,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001512,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

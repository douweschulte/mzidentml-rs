use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnzymeName {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for EnzymeName {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("EnzymeName", "cvParam"));
        }

        self.validate_cv_params(strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "EnzymeName",
                "userParam",
            ));
        }

        for user_param in &self.user_params {
            user_param.validate(strict)?;
        }
        Ok(())
    }
}

has_cv_params!(
    EnzymeName,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001045,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

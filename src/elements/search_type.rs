use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

// TODO: This might not be fully correct. SearchType is supposed to habe 1 CvParam and 1 UserParam. It might mis similar to SoftwareName.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchType {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for SearchType {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("SearchType", "cvParam"));
        }
        self.validate_cv_params(strict)?;

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }
        Ok(())
    }
}

has_cv_params!(
    SearchType,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001080,
        occurence: CvParamOccurence::MustOnceOrMany,
        supplies_children: true,
    },]
);

use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilterType {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for FilterType {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.len() != 1 {
            return Err(ValidationError::ChildRequiredOnce("FilterType", "cvParam"));
        }

        self.validate_cv_params(strict)?;

        if self.user_params.len() != 1 {
            return Err(ValidationError::ChildRequiredOnce(
                "FilterType",
                "userParam",
            ));
        }

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    FilterType,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001511,
        occurence: CvParamOccurence::MustOnceOrMany,
        supplies_children: true,
    },]
);

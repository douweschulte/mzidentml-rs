use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalSearchParams {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for AdditionalSearchParams {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AdditionalSearchParams",
                "cvParam",
            ));
        }
        self.validate_cv_params(strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AdditionalSearchParams",
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
    AdditionalSearchParams,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001302,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001066,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001210,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1002489,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        }
    ]
);

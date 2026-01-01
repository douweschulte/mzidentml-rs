use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareName {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>, // TODO: Actually only one is allowed, but has_cv_params! accept only vec fields now.
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for SoftwareName {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "SoftwareName",
                "cvParam",
            ));
        }
        self.validate_cv_params(strict)?;

        if !self.cv_param_by_accession("MS", 1000799)?.is_empty() {
            return Err(ValidationError::ReasonedChildRequiredOnce(
                "SoftwareName",
                "userName",
                "MS:1000799 a userParam with the name of the software is needed.",
            ));
        }

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    SoftwareName,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001456,
        occurence: CvParamOccurence::MustOnceOrMany,
        supplies_children: true,
    },]
);

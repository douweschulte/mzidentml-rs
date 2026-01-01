use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_param,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "cvParam")]
    pub cv_param: CvParam,
}

impl IsElement for Role {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        self.validate_cv_params(strict)?;
        Ok(())
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

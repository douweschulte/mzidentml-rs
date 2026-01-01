use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement, user_param::UserParam},
    error::ValidationError,
    has_opt_cv_param,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseName {
    #[serde(default, rename = "cvParam")]
    pub cv_param: Option<CvParam>,
    #[serde(rename = "userParam")]
    pub user_param: Option<UserParam>,
}

impl IsElement for DatabaseName {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_param.is_some() && self.user_param.is_some() {
            return Err(ValidationError::ExclisiveAttribute(
                "DatabaseName",
                &["cvParam", "userParam"],
            ));
        }
        self.validate_cv_params(strict)?;

        if let Some(param) = &self.user_param {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_opt_cv_param!(
    DatabaseName,
    cv_param,
    [CvParamRule {
        cv_name: "MS",
        id: 1001013,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    }]
);

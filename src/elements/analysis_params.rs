use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisParams {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for AnalysisParams {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AnalysisParams",
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AnalysisParams",
                "userParam",
            ));
        }

        for param in self.user_params.iter() {
            param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    AnalysisParams,
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
            id: 1001194,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
    ]
);

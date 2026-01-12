use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecificityRules {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for SpecificityRules {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "SpecificityRules",
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict)?;

        Ok(())
    }
}

has_cv_params!(
    SpecificityRules,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001056,
        occurence: CvParamOccurence::MustOnce,
        supplies_children: true,
    },]
);

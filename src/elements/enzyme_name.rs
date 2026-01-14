use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, user_param::UserParam,
    },
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
    const ELEMENT_TAG: &str = "EnzymeName";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict, element_path)?;

        if self.user_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "userParam",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.user_params.iter())?;
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

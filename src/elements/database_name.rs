use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_opt_cv_param,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseName {
    #[serde(default, rename = "cvParam")]
    pub cv_param: Option<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_param: Option<UserParam>,
}

impl IsElement for DatabaseName {
    const ELEMENT_TAG: &str = "DatabaseName";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.cv_param.is_some() && self.user_param.is_some() {
            return Err(ValidationError::ExclisiveAttribute(
                Self::element_path_to_string(element_path),
                &["cvParam", "userParam"],
            ));
        }
        self.validate_cv_params(version, strict, element_path)?;

        if let Some(param) = &self.user_param {
            param.validate(version, strict, element_path, None)?;
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

use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileFormat {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for FileFormat {
    const ELEMENT_TAG: &str = "FileFormat";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.cv_params.len() != 1 {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(
    FileFormat,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001040,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1000560,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001347,
            occurence: CvParamOccurence::MustOnceOrMany,
            supplies_children: true,
        },
    ]
);

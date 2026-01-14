use serde::{Deserialize, Serialize};

use crate::elements::attributes::semver::SemVer;
use crate::error::ValidationError;
use crate::{
    elements::{cv_param::CvParam, is_element::IsElement},
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIDFormat {
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for SpectrumIDFormat {
    const ELEMENT_TAG: &str = "SpectrumIDFormat";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.cv_params.len() != 1 {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "cvParams",
            ));
        }
        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(
    SpectrumIDFormat,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1000767,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001529,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: true,
        },
    ]
);

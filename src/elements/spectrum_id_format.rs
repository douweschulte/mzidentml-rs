use serde::{Deserialize, Serialize};

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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.len() != 1 {
            return Err(ValidationError::ChildRequiredOnce(
                "SpectrumIDFormat",
                "cvParams",
            ));
        }
        self.validate_cv_params(strict)
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

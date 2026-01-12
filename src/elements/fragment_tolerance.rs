use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct FragmentTolerance {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for FragmentTolerance {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "FragmentTolerance",
                "cvParam",
            ));
        }

        self.validate_cv_params(strict)?;
        Ok(())
    }
}

has_cv_params!(
    FragmentTolerance,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001412,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001413,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        }
    ]
);

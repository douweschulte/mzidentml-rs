use serde::{Deserialize, Serialize};

use crate::{
    elements::{cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileFormat {
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for FileFormat {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.cv_params.len() != 1 {
            return Err(ValidationError::ChildRequiredOnce("FileFormat", "cvParam"));
        }

        self.validate_cv_params(strict)
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

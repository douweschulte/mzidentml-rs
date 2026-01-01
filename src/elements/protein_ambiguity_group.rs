use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        cv_param::CvParam, is_element::IsElement,
        protein_detection_hypothesis::ProteinDetectionHypothesis, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProteinAmbiguityGroup {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(default, rename = "ProteinDetectionHypothesis")]
    pub protein_detection_hypotheses: Vec<ProteinDetectionHypothesis>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for ProteinAmbiguityGroup {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "ProteinAmbiguityGroup",
                "id",
            ));
        }

        if self.protein_detection_hypotheses.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "ProteinAmbiguityGroup",
                "ProteinDetectionHypothesis",
            ));
        }

        for protein_detection_hypothesis in &self.protein_detection_hypotheses {
            protein_detection_hypothesis.validate(strict)?;
        }

        self.validate_cv_params(strict)?;

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    ProteinAmbiguityGroup,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1002415,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001147,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
    ]
);

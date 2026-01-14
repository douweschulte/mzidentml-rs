use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement,
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
    const ELEMENT_TAG: &str = "ProteinAmbiguityGroup";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        if self.protein_detection_hypotheses.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "ProteinDetectionHypothesis",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.protein_detection_hypotheses.iter(),
        )?;

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())
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

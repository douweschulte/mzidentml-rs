use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement,
        peptide_hypothesis::PeptideHypothesis, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProteinDetectionHypothesis {
    #[serde(rename = "@dBSequence_ref")]
    pub db_sequence_ref: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@passThreshold")]
    pub pass_threshold: bool,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(default, rename = "PeptideHypothesis")]
    pub peptide_hypotheses: Vec<PeptideHypothesis>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for ProteinDetectionHypothesis {
    const ELEMENT_TAG: &str = "ProteinDetectionHypothesis";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.db_sequence_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "dbSequence_ref",
            ));
        }
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        if self.peptide_hypotheses.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "PeptideHypothesis",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.peptide_hypotheses.iter(),
        )?;

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())
    }
}

has_cv_params!(
    ProteinDetectionHypothesis,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1002403,
            occurence: CvParamOccurence::MayOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1002402,
            occurence: CvParamOccurence::MayOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1002401,
            occurence: CvParamOccurence::MayOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001116,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001153,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001060,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001101,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1002664,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
    ]
);

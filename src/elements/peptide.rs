use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement,
        modification::Modification, peptide_sequence::PeptideSequence,
        substitution_modification::SubstitutionModification, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peptide {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "PeptideSequence")]
    pub peptide_sequence: PeptideSequence,
    #[serde(default, rename = "Modification")]
    pub modification: Vec<Modification>,
    #[serde(default, rename = "SubstitutionModification")]
    pub substitution_modification: Vec<SubstitutionModification>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for Peptide {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("Peptide", "id"));
        }

        self.peptide_sequence.validate(version, strict)?;

        for modification in self.modification.iter() {
            modification.validate(version, strict)?;
        }

        for modification in self.substitution_modification.iter() {
            modification.validate(version, strict)?;
        }

        self.validate_cv_params(version, strict)?;
        for user_param in &self.user_params {
            user_param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    Peptide,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001355,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

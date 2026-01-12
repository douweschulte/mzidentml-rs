use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        ambiguous_residue::AmbiguousResidue, cv_param::CvParam, is_element::IsElement,
        residue::Residue, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MassTable {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@msLevel")]
    pub ms_level: usize,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "Residue")]
    pub residues: Vec<Residue>,
    #[serde(rename = "AmbiguousResidue")]
    pub ambiguous_residues: Vec<AmbiguousResidue>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for MassTable {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("MassTable", "id"));
        }

        self.validate_cv_params(strict)?;

        for param in &self.user_params {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    MassTable,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001354,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

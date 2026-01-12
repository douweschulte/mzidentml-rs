use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, fragmentation::Fragmentation,
        is_element::IsElement, peptide_evidence_ref::PeptideEvidenceRef, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationItem {
    #[serde(rename = "@chargeState")]
    pub charge_state: usize,
    #[serde(rename = "@experimentalMassToCharge")]
    pub experimental_mass_to_charge: f64,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@passThreshold")]
    pub pass_threshold: bool,
    #[serde(rename = "@peptide_ref")]
    pub peptide_ref: String,
    #[serde(rename = "@rank")]
    pub rank: usize,

    #[serde(rename = "@massTable_ref")]
    pub mass_table_ref: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@sample_ref")]
    pub sample_ref: Option<String>,

    #[serde(default, rename = "PeptideEvidenceRef")]
    pub peptide_evidence_refs: Vec<PeptideEvidenceRef>,
    #[serde(rename = "Fragmentation")]
    pub fragmentation: Option<Fragmentation>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for SpectrumIdentificationItem {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationItem",
                "id",
            ));
        }
        if self.peptide_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationItem",
                "peptide_ref",
            ));
        }

        for evidence in &self.peptide_evidence_refs {
            evidence.validate(version, strict)?;
        }

        for fragmentation in self.fragmentation.iter() {
            fragmentation.validate(version, strict)?;
        }

        self.validate_cv_params(version, strict)?;

        for param in self.user_params.iter() {
            param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    SpectrumIdentificationItem,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001405,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

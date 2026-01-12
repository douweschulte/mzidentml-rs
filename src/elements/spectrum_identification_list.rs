use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, fragmentation_table::FragmentationTable,
        is_element::IsElement, spectrum_identification_result::SpectrumIdentificationResult,
        user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationList {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@numSequencesSearched")]
    pub num_sequences_searched: Option<usize>,

    #[serde(default, rename = "SpectrumIdentificationResult")]
    pub spectrum_identification_results: Vec<SpectrumIdentificationResult>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,

    #[serde(rename = "FragmentationTable")]
    pub fragmentation_table: Option<FragmentationTable>,
}

impl IsElement for SpectrumIdentificationList {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationList",
                "id",
            ));
        }

        if self.spectrum_identification_results.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "SpectrumIdentificationList",
                "SpectrumIdentificationResult",
            ));
        }

        for spectrum_identification_result in &self.spectrum_identification_results {
            spectrum_identification_result.validate(version, strict)?;
        }

        self.validate_cv_params(version, strict)?;

        for param in self.user_params.iter() {
            param.validate(version, strict)?;
        }

        if let Some(fragmentation_table) = &self.fragmentation_table {
            fragmentation_table.validate(version, strict)?;
        }
        Ok(())
    }
}

has_cv_params!(SpectrumIdentificationList, cv_params);

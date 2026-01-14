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
    const ELEMENT_TAG: &str = "SpectrumIdentificationList";

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

        if self.spectrum_identification_results.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentificationResult",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification_results.iter(),
        )?;

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())?;

        if let Some(fragmentation_table) = &self.fragmentation_table {
            fragmentation_table.validate(version, strict, element_path, None)?;
        }
        Ok(())
    }
}

has_cv_params!(SpectrumIdentificationList, cv_params);

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, input_spectra::InputSpectra, is_element::IsElement,
        search_database_ref::SearchDatabaseRef,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentification {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@spectrumIdentificationList_ref")]
    pub spectrum_identification_list_ref: String,
    #[serde(rename = "@spectrumIdentificationProtocol_ref")]
    pub spectrum_identification_protocol_ref: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "InputSpectra")]
    pub input_spectra: Vec<InputSpectra>,
    #[serde(rename = "SearchDatabaseRef")]
    pub search_database_refs: Vec<SearchDatabaseRef>,
}

impl IsElement for SpectrumIdentification {
    const ELEMENT_TAG: &str = "SpectrumIdentification";

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
        if self.spectrum_identification_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "spectrumIdentificationList_ref",
            ));
        }
        if self.spectrum_identification_protocol_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "spectrumIdentificationProtocol_ref",
            ));
        }
        if self.input_spectra.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "InputSpectra",
            ));
        }
        if self.search_database_refs.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SearchDatabaseRef",
            ));
        }
        Self::validate_elements(
            version,
            strict,
            element_path,
            self.search_database_refs.iter(),
        )?;
        Ok(())
    }
}

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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentification",
                "id",
            ));
        }
        if self.spectrum_identification_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentification",
                "spectrumIdentificationList_ref",
            ));
        }
        if self.spectrum_identification_protocol_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentification",
                "spectrumIdentificationProtocol_ref",
            ));
        }
        if self.input_spectra.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "SpectrumIdentification",
                "InputSpectra",
            ));
        }
        if self.search_database_refs.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "SpectrumIdentification",
                "SearchDatabaseRef",
            ));
        }
        for search_db_ref in self.search_database_refs.iter() {
            search_db_ref.validate(version, strict)?;
        }
        Ok(())
    }
}

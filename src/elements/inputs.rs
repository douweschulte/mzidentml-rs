use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement, search_database::SearchDatabase,
        source_file::SourceFile, spectra_data::SpectraData,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    #[serde(default, rename = "SourceFile")]
    source_files: Vec<SourceFile>,
    #[serde(default, rename = "SearchDatabase")]
    search_databases: Vec<SearchDatabase>,
    #[serde(default, rename = "SpectraData")]
    spectra_data: Vec<SpectraData>,
}

impl IsElement for Inputs {
    const ELEMENT_TAG: &str = "Inputs";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(version, strict, element_path, self.source_files.iter())?;
        Self::validate_elements(version, strict, element_path, self.search_databases.iter())?;

        if self.spectra_data.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectraData",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.spectra_data.iter())
    }
}

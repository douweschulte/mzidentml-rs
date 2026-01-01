use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        is_element::IsElement, search_database::SearchDatabase, source_file::SourceFile,
        spectra_data::SpectraData,
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        for source_file in &self.source_files {
            source_file.validate(strict)?;
        }
        for search_database in &self.search_databases {
            search_database.validate(strict)?;
        }

        if self.spectra_data.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Inputs", "SpectraData"));
        }
        for spectra_data in &self.spectra_data {
            spectra_data.validate(strict)?;
        }
        Ok(())
    }
}

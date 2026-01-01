use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        additional_search_params::AdditionalSearchParams, database_filters::DatabaseFilters,
        database_translation::DatabaseTranslation, enzymes::Enzymes,
        fragment_tolerance::FragmentTolerance, is_element::IsElement, mass_table::MassTable,
        modification_params::ModificationParams, parent_tolerance::ParentTolerance,
        search_type::SearchType, threshold::Threshold,
    },
    error::ValidationError,
};

// https://raw.githubusercontent.com/HUPO-PSI/mzIdentML/2aacf89e164afc96f71dee7e433c055718d7db0d/specification_document-releases/specdoc1_3/mzIdentML1.3.0-release.pdf#[{%22num%22%3A254%2C%22gen%22%3A0}%2C{%22name%22%3A%22XYZ%22}%2C192.2%2C251.7%2C0]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationProtocol {
    #[serde(rename = "@analysisSoftware_ref")]
    pub analysis_software_ref: String,
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "SearchType")]
    pub search_type: SearchType,
    #[serde(rename = "AdditionalSearchParams")]
    pub additional_search_params: Option<AdditionalSearchParams>,
    #[serde(rename = "ModificationParams")]
    pub modification_params: Option<ModificationParams>,
    #[serde(rename = "Enzymes")]
    pub enzymes: Option<Enzymes>,
    #[serde(default, rename = "MassTable")]
    pub mass_tables: Vec<MassTable>,
    #[serde(rename = "FragmentTolerance")]
    pub fragment_tolerance: Option<FragmentTolerance>,
    #[serde(rename = "ParentTolerance")]
    pub parent_tolerance: Option<ParentTolerance>,
    #[serde(rename = "Threshold")]
    pub threashold: Threshold,
    #[serde(rename = "DatabaseFilters")]
    pub database_filters: Option<DatabaseFilters>,
    #[serde(rename = "DatabaseTranslation")]
    pub database_translation: Option<DatabaseTranslation>,
}

impl IsElement for SpectrumIdentificationProtocol {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.analysis_software_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationProtocol",
                "analysisSoftware_ref",
            ));
        }
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationProtocol",
                "id",
            ));
        }
        self.search_type.validate(strict)?;
        if let Some(additional_search_params) = &self.additional_search_params {
            additional_search_params.validate(strict)?;
        }
        if let Some(modification_params) = &self.modification_params {
            modification_params.validate(strict)?;
        }
        if let Some(enzymes) = &self.enzymes {
            enzymes.validate(strict)?;
        }
        for mass_table in &self.mass_tables {
            mass_table.validate(strict)?;
        }
        if let Some(fragment_tolerance) = &self.fragment_tolerance {
            fragment_tolerance.validate(strict)?;
        }
        if let Some(parent_tolerance) = &self.parent_tolerance {
            parent_tolerance.validate(strict)?;
        }
        self.threashold.validate(strict)?;
        if let Some(database_filters) = &self.database_filters {
            database_filters.validate(strict)?;
        }
        if let Some(database_translation) = &self.database_translation {
            database_translation.validate(strict)?;
        }
        Ok(())
    }
}

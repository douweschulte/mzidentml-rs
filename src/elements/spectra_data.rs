use serde::{Deserialize, Serialize};

use crate::elements::attributes::semver::SemVer;
use crate::elements::external_format_documentation::ExternalFormatDocumentation;
use crate::elements::file_format::FileFormat;
use crate::elements::is_element::IsElement;
use crate::elements::spectrum_id_format::SpectrumIDFormat;
use crate::error::ValidationError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectraData {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@location")]
    pub location: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "ExternalFormatDocumentation")]
    pub external_format_documentation: Option<ExternalFormatDocumentation>,
    #[serde(rename = "FileFormat")]
    pub file_format: Option<FileFormat>,
    #[serde(rename = "SpectrumIDFormat")]
    pub spectrum_id_format: SpectrumIDFormat,
}

impl IsElement for SpectraData {
    const ELEMENT_TAG: &str = "SpectraData";

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
        if self.location.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "location",
            ));
        }

        if let Some(external_format_documentation) = &self.external_format_documentation {
            external_format_documentation.validate(version, strict, element_path, None)?;
        }

        if let Some(file_format) = self.file_format.as_ref() {
            file_format.validate(version, strict, element_path, None)?;
        } else if version.minor() >= 2 {
            // File format was made mendatory in 1.2
            return Err(ValidationError::MissingChild(
                Self::element_path_to_string(element_path),
                "FileFormat",
            ));
        }

        self.spectrum_id_format
            .validate(version, strict, element_path, None)
    }
}

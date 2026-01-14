use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::elements::attributes::semver::SemVer;
use crate::elements::database_name::DatabaseName;
use crate::elements::external_format_documentation::ExternalFormatDocumentation;
use crate::elements::file_format::FileFormat;
use crate::error::ValidationError;
use crate::parsing::opt_date_time_parsing;
use crate::{
    elements::{cv_param::CvParam, is_element::IsElement},
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchDatabase {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@location")]
    pub location: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@numDatabaseSequences")]
    pub num_database_sequences: Option<usize>,
    #[serde(rename = "@numResidues")]
    pub num_residues: Option<usize>,
    #[serde(default, rename = "@releaseDate", with = "opt_date_time_parsing")]
    pub release_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "ExternalFormatDocumentation")]
    pub external_format_documentation: Option<ExternalFormatDocumentation>,
    #[serde(rename = "FileFormat")]
    pub file_format: Option<FileFormat>,
    #[serde(rename = "DatabaseName")]
    pub database_name: DatabaseName,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for SearchDatabase {
    const ELEMENT_TAG: &str = "SearchDatabase";

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

        self.database_name
            .validate(version, strict, element_path, None)?;

        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(
    SearchDatabase,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1000561,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001011,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
    ]
);

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam,
        external_format_documentation::ExternalFormatDocumentation, file_format::FileFormat,
        is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceFile {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@location")]
    location: String,

    #[serde(rename = "@name")]
    name: Option<String>,

    #[serde(rename = "ExternalFormatDocumentation")]
    external_format_documentation: Option<ExternalFormatDocumentation>,
    #[serde(rename = "FileFormat")]
    file_format: Option<FileFormat>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for SourceFile {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("SourceFile", "id"));
        }
        if self.location.is_empty() {
            return Err(ValidationError::EmptyAttribute("SourceFile", "location"));
        }

        if let Some(external_format_documentation) = &self.external_format_documentation {
            external_format_documentation.validate(version, strict)?;
        }

        if let Some(file_format) = self.file_format.as_ref() {
            file_format.validate(version, strict)?;
        } else if version.minor() >= 2 {
            // File format is introduced in this element in version 1.2
            return Err(ValidationError::MissingChild("SourceFile", "FileFormat"));
        }

        self.validate_cv_params(version, strict)?;

        for param in self.user_params.iter() {
            param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    SourceFile,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1000561,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

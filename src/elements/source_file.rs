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
    const ELEMENT_TAG: &str = "SourceFile";

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
            // File format is introduced in this element in version 1.2
            return Err(ValidationError::MissingChild(
                Self::element_path_to_string(element_path),
                "FileFormat",
            ));
        }

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())
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

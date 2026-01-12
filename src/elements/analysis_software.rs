use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, contact_role::ContactRole, customizations::Customizations,
        is_element::IsElement, software_name::SoftwareName,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisSoftware {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@uri")]
    pub uri: Option<String>,
    #[serde(rename = "ContactRole")]
    pub contact_role: Option<ContactRole>,
    #[serde(rename = "SoftwareName")]
    pub software_name: SoftwareName,
    #[serde(rename = "Customizations")]
    pub customizations: Option<Customizations>,
}

impl IsElement for AnalysisSoftware {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if let Some(contact_role) = &self.contact_role {
            contact_role.validate(version, strict)?;
        }
        if let Some(customizations) = &self.customizations {
            customizations.validate(version, strict)?;
        }
        self.software_name.validate(version, strict)?;
        Ok(())
    }
}

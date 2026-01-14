use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, contact_role::ContactRole, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@analysisSoftware_ref")]
    pub analysis_software_ref: Option<String>, // TODO: needs to be validated
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@ContactRole")]
    pub contact_role: Option<ContactRole>,
}

impl IsElement for Provider {
    const ELEMENT_TAG: &str = "Provider";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if let Some(contact_role) = &self.contact_role {
            contact_role.validate(version, strict, element_path, None)?;
        }
        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement, organization::Organization,
        person::Person,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditCollection {
    #[serde(default, rename = "Person")]
    pub persons: Vec<Person>,
    #[serde(default, rename = "Organization")]
    pub organizations: Vec<Organization>,
}

impl IsElement for AuditCollection {
    const ELEMENT_TAG: &str = "AuditCollection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        // Since version 1.3 person and organization is allowed once or multiple times, before each was only allowed once.
        if version.minor() > 2 {
            if self.persons.is_empty() {
                return Err(ValidationError::ChildRequiredAtLeastOnce(
                    Self::element_path_to_string(element_path),
                    "Person",
                ));
            }

            if self.organizations.is_empty() {
                return Err(ValidationError::ChildRequiredAtLeastOnce(
                    Self::element_path_to_string(element_path),
                    "Organization",
                ));
            }
        } else {
            if self.persons.len() != 1 {
                return Err(ValidationError::ChildRequiredOnce(
                    "AuditCollection",
                    "Person",
                ));
            }

            if self.organizations.len() != 1 {
                return Err(ValidationError::ChildRequiredOnce(
                    "AuditCollection",
                    "Organization",
                ));
            }
        }

        Self::validate_elements(version, strict, element_path, self.persons.iter())?;
        Self::validate_elements(version, strict, element_path, self.organizations.iter())
    }
}

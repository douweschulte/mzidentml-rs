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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        // Since version 1.3 person and organization is allowed once or multiple times, before each was only allowed once.
        if version.minor() > 2 {
            if self.persons.is_empty() {
                return Err(ValidationError::ChildRequiredAtLeastOnce(
                    "AuditCollection",
                    "Person",
                ));
            }

            if self.organizations.is_empty() {
                return Err(ValidationError::ChildRequiredAtLeastOnce(
                    "AuditCollection",
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

        for person in &self.persons {
            person.validate(version, strict)?;
        }

        for organization in &self.organizations {
            organization.validate(version, strict)?;
        }
        Ok(())
    }
}

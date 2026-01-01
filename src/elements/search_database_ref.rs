use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchDatabaseRef {
    #[serde(rename = "@searchDatabase_ref")]
    pub search_database_ref: String,
}

impl IsElement for SearchDatabaseRef {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.search_database_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SearchDatabaseRef",
                "spectraData_ref",
            ));
        }
        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSpectra {
    #[serde(rename = "@spectraData_ref")]
    pub spectra_data_ref: String,
}

impl IsElement for InputSpectra {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.spectra_data_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "InputSpectra",
                "spectraData_ref",
            ));
        }
        Ok(())
    }
}

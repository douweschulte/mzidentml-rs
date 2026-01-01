use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalFormatDocumentation(String);

impl IsElement for ExternalFormatDocumentation {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

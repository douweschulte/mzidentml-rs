use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customizations(String);

impl IsElement for Customizations {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

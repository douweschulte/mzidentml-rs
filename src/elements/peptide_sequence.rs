use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeptideSequence(String);

impl IsElement for PeptideSequence {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

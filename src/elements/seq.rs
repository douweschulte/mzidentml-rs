use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Seq(String);

impl IsElement for Seq {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

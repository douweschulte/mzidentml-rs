use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteRegexp(String);

impl IsElement for SiteRegexp {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}

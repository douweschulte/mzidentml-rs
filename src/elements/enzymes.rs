use serde::{Deserialize, Serialize};

use crate::{
    elements::{enzyme::Enzyme, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enzymes {
    #[serde(rename = "@independent")]
    pub independent: Option<bool>,

    #[serde(rename = "Enzyme")]
    pub enzyme: Vec<Enzyme>,
}

impl IsElement for Enzymes {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.enzyme.is_empty() {
            return Err(ValidationError::ChildRequiredOnce("Enzymes", "Enzyme"));
        }

        for enz in &self.enzyme {
            enz.validate(strict)?;
        }
        Ok(())
    }
}

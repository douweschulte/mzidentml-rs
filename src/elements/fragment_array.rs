use serde::{Deserialize, Serialize};

use crate::{
    elements::is_element::IsElement, error::ValidationError, parsing::space_separated_vec_parsing,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentArray {
    #[serde(rename = "@measure_ref")]
    pub measure_ref: String,
    #[serde(rename = "@values", with = "space_separated_vec_parsing")]
    pub values: Vec<f64>,
}

impl IsElement for FragmentArray {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.measure_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "FragmentArray",
                "measure_ref",
            ));
        }

        Ok(())
    }
}

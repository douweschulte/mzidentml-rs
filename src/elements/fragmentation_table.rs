use serde::{Deserialize, Serialize};

use crate::{
    elements::{is_element::IsElement, measure::Measure},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentationTable {
    #[serde(rename = "Measure")]
    pub measures: Vec<Measure>,
}

impl IsElement for FragmentationTable {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.measures.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "FragmentationTable",
                "Measure",
            ));
        }

        for measure in &self.measures {
            measure.validate(strict)?;
        }

        Ok(())
    }
}

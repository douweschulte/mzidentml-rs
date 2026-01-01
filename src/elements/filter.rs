use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        exclude::Exclude, filter_type::FilterType, include::Include, is_element::IsElement,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "FilterType")]
    pub filter_type: FilterType,
    #[serde(rename = "Include")]
    pub include: Option<Include>,
    #[serde(rename = "Exclude")]
    pub exclude: Option<Exclude>,
}

impl IsElement for Filter {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        self.filter_type.validate(strict)?;

        if let Some(include) = &self.include {
            include.validate(strict)?;
        }

        if let Some(exclude) = &self.exclude {
            exclude.validate(strict)?;
        }

        Ok(())
    }
}

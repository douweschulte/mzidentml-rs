use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, exclude::Exclude, filter_type::FilterType, include::Include,
        is_element::IsElement,
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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        self.filter_type.validate(version, strict)?;

        if let Some(include) = &self.include {
            include.validate(version, strict)?;
        }

        if let Some(exclude) = &self.exclude {
            exclude.validate(version, strict)?;
        }

        Ok(())
    }
}

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
    const ELEMENT_TAG: &str = "Filter";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        self.filter_type
            .validate(version, strict, element_path, None)?;

        if let Some(include) = &self.include {
            include.validate(version, strict, element_path, None)?;
        }

        if let Some(exclude) = &self.exclude {
            exclude.validate(version, strict, element_path, None)?;
        }

        Ok(())
    }
}

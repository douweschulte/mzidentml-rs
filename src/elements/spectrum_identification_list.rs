use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    elements::{
        attributes::semver::SemVer,
        cv_param::CvParam,
        fragmentation_table::FragmentationTable,
        has_cv_params::{CvParamRule, HasCvParams},
        is_element::IsElement,
        spectrum_identification_result::SpectrumIdentificationResult,
        user_param::UserParam,
    },
    error::ValidationError,
};

pub trait IsSpectrumIdentificationList:
    IsElement + HasCvParams + Clone + std::fmt::Debug + Serialize + DeserializeOwned
{
    const SPECTRUM_IDENTIFICATION_LIST_ELEMENT_TAG: &str = "SpectrumIdentificationList";
    const SPECTRUM_IDENTIFICATION_LIST_CV_PARAM_RULES: &[CvParamRule] = &[];
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationList {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@numSequencesSearched")]
    pub num_sequences_searched: Option<usize>,

    #[serde(default, rename = "SpectrumIdentificationResult")]
    pub spectrum_identification_results: Vec<SpectrumIdentificationResult>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,

    #[serde(rename = "FragmentationTable")]
    pub fragmentation_table: Option<FragmentationTable>,
}

impl IsSpectrumIdentificationList for SpectrumIdentificationList {}

impl IsElement for SpectrumIdentificationList {
    const ELEMENT_TAG: &str = Self::SPECTRUM_IDENTIFICATION_LIST_ELEMENT_TAG;

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        if self.spectrum_identification_results.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentificationResult",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification_results.iter(),
        )?;

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())?;

        if let Some(fragmentation_table) = &self.fragmentation_table {
            fragmentation_table.validate(version, strict, element_path, None)?;
        }
        Ok(())
    }
}

impl HasCvParams for SpectrumIdentificationList {
    const CV_PARAM_RULES: &[CvParamRule] = Self::SPECTRUM_IDENTIFICATION_LIST_CV_PARAM_RULES;

    fn cv_params(&self) -> impl Iterator<Item = &CvParam> {
        self.cv_params.iter()
    }
}

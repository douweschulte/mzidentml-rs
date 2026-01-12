use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        cv_param::CvParam, is_element::IsElement,
        spectrum_identification_item::SpectrumIdentificationItem, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationResult {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@spectraData_ref")]
    pub spectra_data_ref: String,
    #[serde(rename = "@spectrumID")]
    pub spectrum_id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "SpectrumIdentificationItem")]
    pub spectrum_identification_items: Vec<SpectrumIdentificationItem>,

    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,

    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for SpectrumIdentificationResult {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "id",
            ));
        }

        if self.spectra_data_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "spectraData_ref",
            ));
        }

        if self.spectrum_identification_items.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "SpectrumIdentificationItem",
            ));
        }

        for item in &self.spectrum_identification_items {
            item.validate(strict)?;
        }

        self.validate_cv_params(strict)?;

        for param in self.user_params.iter() {
            param.validate(strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    SpectrumIdentificationResult,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001405,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true
    }]
);

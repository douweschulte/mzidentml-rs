use std::{
    collections::HashMap,
    io::{BufRead, Seek, SeekFrom},
};

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer,
        cv_param::CvParam,
        fragmentation_table::FragmentationTable,
        has_cv_params::{CvParamRule, HasCvParams},
        is_element::IsElement,
        spectrum_identification_list::IsSpectrumIdentificationList,
        spectrum_identification_result::SpectrumIdentificationResult,
        user_param::UserParam,
    },
    error::{ReadIndexedError, ValidationError},
    indexed_elements::is_indexed_element::IsIndexedElement,
};

/// Struct keeping unindexed data and maps to SpectrumIdentificationResults
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationList {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@numSequencesSearched")]
    pub num_sequences_searched: Option<usize>,

    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,

    #[serde(rename = "FragmentationTable")]
    pub fragmentation_table: Option<FragmentationTable>,

    #[serde(default, rename = "SpectrumIdentificationResult")]
    pub spectrum_identification_results_map: HashMap<String, u64>,
}

impl SpectrumIdentificationList {
    fn spectrum_identification_result_by_offset<R: BufRead + Seek>(
        &self,
        reader: &mut R,
        offset: &u64,
    ) -> Result<SpectrumIdentificationResult, ReadIndexedError> {
        reader
            .seek(SeekFrom::Start(*offset))
            .map_err(|err| ReadIndexedError::Seek(*offset, format!("{err:?}")))?;

        let spectrum_ident_result =
            quick_xml::de::from_reader::<_, SpectrumIdentificationResult>(reader).map_err(
                |err| ReadIndexedError::Deserialize("SpectrumIdentificationResult", *offset, err),
            )?;

        Ok(spectrum_ident_result)
    }

    pub fn spectrum_identification_result_by_id<R: BufRead + Seek>(
        &self,
        reader: &mut R,
        id: &str,
    ) -> Result<SpectrumIdentificationResult, ReadIndexedError> {
        let offset = self
            .spectrum_identification_results_map
            .get(id)
            .ok_or(ReadIndexedError::UnknownIdentifier)?;

        self.spectrum_identification_result_by_offset(reader, offset)
    }

    pub fn spectrum_identification_results<R: BufRead + Seek>(
        &self,
        reader: &mut R,
    ) -> impl Iterator<Item = Result<SpectrumIdentificationResult, ReadIndexedError>> {
        self.spectrum_identification_results_map
            .values()
            .map(|offset| self.spectrum_identification_result_by_offset(reader, offset))
    }
}

impl IsSpectrumIdentificationList for SpectrumIdentificationList {}

impl IsElement for SpectrumIdentificationList {
    const ELEMENT_TAG: &str = "SpectrumIdentificationList";

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

        if self.spectrum_identification_results_map.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentificationResult",
            ));
        }

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

impl IsIndexedElement for SpectrumIdentificationList {
    fn inner_validate_indexed<R: BufRead + Seek>(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
        reader: &mut R,
    ) -> Result<(), ValidationError> {
        for (elem_idx, elem) in self.spectrum_identification_results(reader).enumerate() {
            elem?.validate(version, strict, element_path, Some(elem_idx))?;
        }
        Ok(())
    }
}

use std::{
    collections::HashMap,
    fmt::Debug,
    io::{BufRead, Seek, SeekFrom},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    elements::{
        attributes::semver::SemVer, db_sequence::DbSequence, is_element::IsElement,
        peptide::Peptide, peptide_evidence::PeptideEvidence,
        sequence_collection::IsSequenceCollection,
    },
    error::{ReadIndexedError, ValidationError},
    indexed_elements::is_indexed_element::IsIndexedElement,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequenceCollection {
    #[serde(default, rename = "DBSequence")]
    pub db_sequences_map: HashMap<String, u64>,
    #[serde(default, rename = "Peptide")]
    pub peptides_map: HashMap<String, u64>,
    #[serde(default, rename = "PeptideEvidence")]
    pub peptide_evidence_map: HashMap<String, u64>,
}

impl SequenceCollection {
    fn element_by_offset<R: BufRead + Seek, T: DeserializeOwned>(
        &self,
        reader: &mut R,
        offset: &u64,
    ) -> Result<T, ReadIndexedError> {
        reader
            .seek(SeekFrom::Start(*offset))
            .map_err(|err| ReadIndexedError::Seek(*offset, format!("{err:?}")))?;

        let spectrum_ident_result = quick_xml::de::from_reader::<_, T>(reader).map_err(|err| {
            ReadIndexedError::Deserialize("SpectrumIdentificationResult", *offset, err)
        })?;

        Ok(spectrum_ident_result)
    }

    pub fn db_sequence_by_id<R: BufRead + Seek>(
        &self,
        reader: &mut R,
        id: &str,
    ) -> Result<DbSequence, ReadIndexedError> {
        let offset = self
            .db_sequences_map
            .get(id)
            .ok_or(ReadIndexedError::UnknownIdentifier)?;

        self.element_by_offset(reader, offset)
    }

    pub fn db_sequences<R: BufRead + Seek>(
        &self,
        reader: &mut R,
    ) -> impl Iterator<Item = Result<DbSequence, ReadIndexedError>> {
        self.db_sequences_map
            .values()
            .map(|offset| self.element_by_offset(reader, offset))
    }

    pub fn peptides_by_id<R: BufRead + Seek>(
        &self,
        reader: &mut R,
        id: &str,
    ) -> Result<Peptide, ReadIndexedError> {
        let offset = self
            .peptides_map
            .get(id)
            .ok_or(ReadIndexedError::UnknownIdentifier)?;

        self.element_by_offset(reader, offset)
    }

    pub fn peptides<R: BufRead + Seek>(
        &self,
        reader: &mut R,
    ) -> impl Iterator<Item = Result<Peptide, ReadIndexedError>> {
        self.peptides_map
            .values()
            .map(|offset| self.element_by_offset(reader, offset))
    }

    pub fn peptide_evidence_by_id<R: BufRead + Seek>(
        &self,
        reader: &mut R,
        id: &str,
    ) -> Result<PeptideEvidence, ReadIndexedError> {
        let offset = self
            .peptide_evidence_map
            .get(id)
            .ok_or(ReadIndexedError::UnknownIdentifier)?;

        self.element_by_offset(reader, offset)
    }

    pub fn peptide_evidences<R: BufRead + Seek>(
        &self,
        reader: &mut R,
    ) -> impl Iterator<Item = Result<PeptideEvidence, ReadIndexedError>> {
        self.peptide_evidence_map
            .values()
            .map(|offset| self.element_by_offset(reader, offset))
    }
}

impl IsSequenceCollection for SequenceCollection {}

impl IsElement for SequenceCollection {
    const ELEMENT_TAG: &str = Self::SEQEUNCE_COLLECTION_ELEMENT_TAG;

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        _element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl IsIndexedElement for SequenceCollection {
    fn inner_validate_indexed<R: BufRead + Seek>(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
        reader: &mut R,
    ) -> Result<(), ValidationError> {
        for (elem_idx, elem) in self.db_sequences(reader).enumerate() {
            elem?.validate(version, strict, element_path, Some(elem_idx))?;
        }
        for (elem_idx, elem) in self.peptides(reader).enumerate() {
            elem?.validate(version, strict, element_path, Some(elem_idx))?;
        }
        for (elem_idx, elem) in self.peptide_evidences(reader).enumerate() {
            elem?.validate(version, strict, element_path, Some(elem_idx))?;
        }
        Ok(())
    }
}

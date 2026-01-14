use std::{fmt::Display, sync::Arc};

use thiserror::Error;

use crate::elements::has_cv_params::{CvParamOccurence, CvParamRule};

/// Things which can got wrong working with mzIdentML files.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error `{}` at element MzIdentML.{}", .0.inner(), .0.path().to_string().trim_start_matches('.'))]
    Deserialization(#[from] serde_path_to_error::Error<quick_xml::DeError>),
    #[error("{0}")]
    Cv(#[from] CvError),
    #[error("{0}")]
    Validation(#[from] ValidationError),
    #[error("{0}")]
    Indexing(#[from] IndexingError),
}

#[derive(Clone, Debug, Error)]
pub enum ValidationError {
    #[error("Wrong cvParam rule, got {0} expected {1}")]
    WrongCvParamOccurence(CvParamOccurence, CvParamOccurence),
    #[error("{0}")]
    CvParamViolation(#[from] CvParamsValidationError),
    #[error("{0} > {1} is required at least once")]
    ChildRequiredOnce(&'static str, &'static str),
    #[error("{0} > {1} is required at least once")]
    ChildRequiredAtLeastOnce(&'static str, &'static str),
    #[error("{0} > {1} is required at least once, due to {2}")]
    ReasonedChildRequiredAtLeastOnce(&'static str, &'static str, &'static str),
    #[error("Missing element {0}")]
    MissingElement(&'static str),
    #[error("{0}[{1}] cannot be empty")]
    EmptyAttribute(&'static str, &'static str),
    #[error("{0} > ({1:?}) is allowed at a time")]
    ExclisiveAttribute(&'static str, &'static [&'static str]),
    #[error("{0}[{1}] has invalid expected `{2}`")]
    InvalidAttributeValue(&'static str, &'static str, String),
    #[error("Unable to parse version {0}, expected `major.minor.patch`")]
    InvalidVersion(String),
    #[error("{0}")]
    Cv(#[from] CvError),
    #[error("{0} > {1} is missing")]
    MissingChild(&'static str, &'static str),
}

/// Error for violated CvParam rules
#[derive(Clone, Debug, Error)]
pub enum CvParamsValidationError {
    RuleViolation(&'static CvParamRule, Option<Vec<String>>),
    Duplication(String, usize),
}

impl Display for CvParamsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CvParamsValidationError::RuleViolation(rule, matching_accessions) => {
                let found = if let Some(matching_accessions) = matching_accessions {
                    matching_accessions.join(", ")
                } else {
                    "none".to_string()
                };
                write!(f, "Violated rule: `{rule}`. Found {found}.")
            }
            CvParamsValidationError::Duplication(cv_name, term_id) => {
                write!(f, "Found duplicate for {cv_name}:{term_id}")
            }
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum CvError {
    #[error("Invalid CV accession format for `{0}`, expected `<ONTHOLOGY>:<ID>`")]
    InvalidIdFormat(String),
    #[error("Got CV `{0}` but could not parse second part of ID to integer `{1}`")]
    InvalidSecondPart(String, String),
    #[error("Unable to initialite CV index for `{0}`, because of the following reasons: {1:?}")]
    IndexInit(
        &'static str,
        Arc<Vec<context_error::BoxedError<'static, mzcv::CVError>>>,
    ),
    #[error("Unknown CV")]
    UnknownCv,
    #[error("Unknown CV term: `{0}:{1}`. Maybe the CV source is outdated?")]
    UnknownCvTerm(String, usize),
}

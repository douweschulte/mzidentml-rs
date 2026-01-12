use std::sync::Arc;

use thiserror::Error;

use crate::elements::has_cv_params::CvParamOccurence;

/// Things which can got wrong working with mzIdentML files.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error `{}` at element MzIdentML.{}", .0.inner(), .0.path().to_string().trim_start_matches('.'))]
    Deserialization(#[from] serde_path_to_error::Error<quick_xml::DeError>),
    #[error("{0}")]
    Cv(#[from] CvError),
    #[error("{0}")]
    Validation(#[from] ValidationError),
}

#[derive(Clone, Debug, Error)]
pub enum ValidationError {
    #[error("Wrong cvParam rule, got {0} expected {1}")]
    WrongCvParamOccurence(CvParamOccurence, CvParamOccurence),
    #[error("One child of `{0}:{1}` must be present.")]
    MustOnceMissing(&'static str, usize),
    #[error("Only one child of `{0}` can be present. Found multiple: `{1:?}`")]
    MustOnceExceeded(&'static str, usize, Vec<String>),
    #[error("At least one child of `{0}:{1}` must be present.")]
    MustOnceOrManyMissing(&'static str, usize),
    #[error("Found duplicate for {0}:{1}")]
    MustOnceOrManyDuplicate(String, usize),
    #[error("Only zero or one child of `{0}:{1}` can be present")]
    MayOnceExceeded(&'static str, usize),
    #[error("Found duplicate for {0}:{1}")]
    MayOnceOrManyDuplicate(String, usize),
    #[error("At least one child of `{0}:{1}` should be present.")]
    ShouldOnceOrManyMissing(&'static str, usize),
    #[error("Found duplicate for {0}:{1}")]
    ShouldOnceOrManyDuplicate(String, usize),
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

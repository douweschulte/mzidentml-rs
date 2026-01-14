use std::collections::HashSet;
use std::fmt::Display;
use std::sync::Arc;

use mzcv::CVData;

use crate::controlled_vocabularies::{CvDataWithChildren, MS_CVINDEX, UNIMOD_CVINDEX};
use crate::elements::attributes::semver::SemVer;
use crate::elements::cv_param::CvParam;
use crate::elements::is_element::IsElement;
use crate::error::{CvError, CvParamsValidationError, ValidationError};

/// Enum representing the number of times a term is allowed to be used in the cvParams of an element.
#[derive(Clone, Debug, PartialEq)]
pub enum CvParamOccurence {
    MustOnce,
    MustOnceOrMany,
    MayOnce,
    MayOnceOrMany,
    ShouldOnceOrMany,
}

impl Display for CvParamOccurence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CvParamOccurence::MustOnce => write!(f, "MustOnce"),
            CvParamOccurence::MustOnceOrMany => write!(f, "MustOnceOrMany"),
            CvParamOccurence::MayOnce => write!(f, "MayOnce"),
            CvParamOccurence::MayOnceOrMany => write!(f, "MayOnceOrMany"),
            CvParamOccurence::ShouldOnceOrMany => write!(f, "ShouldOnceOrMany"),
        }
    }
}

#[derive(Debug)]
pub struct CvParamRule {
    /// CV name, e.g. UNIMOD, MS, .., Check [crate::controlled_vocabularies] for known CVs
    pub cv_name: &'static str,
    /// Second part of the term ID, e.g. MS:1001191 => 1001191
    pub id: usize,
    /// How many times the term may be useed
    pub occurence: CvParamOccurence,
    /// If true the term's children are used
    pub supplies_children: bool,
}

impl CvParamRule {
    /// Returns the children of a given CV Term from the controlled vocabulary
    ///
    fn children_of(
        &self,
        cv_name: &str,
        term_id: &usize,
    ) -> Result<Vec<Arc<CvDataWithChildren>>, ValidationError> {
        match cv_name.to_lowercase().as_str() {
            "ms" => MS_CVINDEX
                .as_ref()
                .map_err(|err| ValidationError::Cv(err.clone()))?
                .children_of(term_id)
                .map_err(ValidationError::Cv),
            "unimod" => UNIMOD_CVINDEX
                .as_ref()
                .map_err(|err| ValidationError::Cv(err.clone()))?
                .children_of(term_id)
                .map_err(ValidationError::Cv),
            _ => Err(CvError::UnknownCv.into()),
        }
    }

    /// Collect cvParams matching the term in the rule.
    ///
    fn collect_matching_cv_params<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_sparams = if self.supplies_children {
            let allowed_term = self.children_of(self.cv_name, &self.id)?;
            let allowed_term_ids: HashSet<usize> = allowed_term
                .iter()
                .filter_map(|term| term.index())
                .collect();

            cv_params
                .filter(|param| param.accession.0 == self.cv_name)
                .filter(|param| allowed_term_ids.contains(&param.accession.1))
                .collect::<Vec<_>>()
        } else {
            cv_params
                .filter(|param| param.accession.0 == self.cv_name && param.accession.1 == self.id)
                .collect::<Vec<_>>()
        };
        Ok(matching_cv_sparams)
    }

    /// Checks if the cvParam (or child) exists once
    ///
    fn validate_must_once<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_params = self.collect_matching_cv_params(cv_params)?;

        if matching_cv_params.is_empty() {
            return Err(CvParamsValidationError::RuleViolation(self, None).into());
        } else if matching_cv_params.len() > 1 {
            return Err(CvParamsValidationError::RuleViolation(
                self,
                Some(
                    matching_cv_params
                        .iter()
                        .map(|param| format!("{}:{}", param.accession.0, param.accession.1))
                        .collect(),
                ),
            )
            .into());
        }

        Ok(matching_cv_params)
    }

    /// Checks if the cvParam (or child) exists at least once and without any duplicates
    ///
    fn validate_must_once_or_many<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_params = self.collect_matching_cv_params(cv_params)?;

        if matching_cv_params.is_empty() {
            return Err(CvParamsValidationError::RuleViolation(self, None).into());
        }

        // Check for duplicates
        let mut visited_ids: HashSet<usize> = HashSet::with_capacity(matching_cv_params.len());
        for param in matching_cv_params.iter() {
            if visited_ids.contains(&param.accession.1) {
                return Err(CvParamsValidationError::Duplication(
                    param.accession.0.clone(),
                    param.accession.1,
                )
                .into());
            } else {
                visited_ids.insert(param.accession.1);
            }
        }

        Ok(matching_cv_params)
    }

    /// Checks if the cvParam (or child) exists only once or not at all.
    ///
    fn validate_may_once<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_params = self.collect_matching_cv_params(cv_params)?;

        if matching_cv_params.len() > 1 {
            return Err(CvParamsValidationError::RuleViolation(
                self,
                Some(
                    matching_cv_params
                        .iter()
                        .map(|param| format!("{}:{}", param.accession.0, param.accession.1))
                        .collect(),
                ),
            )
            .into());
        }

        Ok(matching_cv_params)
    }

    /// Checks if the cvParam (or child) exists without duplicates
    ///
    fn validate_may_once_or_many<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_params = self.collect_matching_cv_params(cv_params)?;

        // Check for duplicates
        let mut visited_ids: HashSet<usize> = HashSet::with_capacity(matching_cv_params.len());
        for param in matching_cv_params.iter() {
            if visited_ids.contains(&param.accession.1) {
                return Err(CvParamsValidationError::Duplication(
                    param.accession.0.clone(),
                    param.accession.1,
                )
                .into());
            } else {
                visited_ids.insert(param.accession.1);
            }
        }

        Ok(matching_cv_params)
    }

    /// Checks if the cvParam (or child) exists at least once and without any duplicates
    ///
    // TODO: This is exactly the same as validate_many_once_or_many except the thrown errors. Can it be merged?
    fn validate_should_once_or_many<'a, I>(
        &'static self,
        cv_params: I,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        let matching_cv_params = self.collect_matching_cv_params(cv_params)?;

        if matching_cv_params.is_empty() {
            return Err(CvParamsValidationError::RuleViolation(self, None).into());
        }

        // Check for duplicates
        let mut visited_ids: HashSet<usize> = HashSet::with_capacity(matching_cv_params.len());
        for param in matching_cv_params.iter() {
            if visited_ids.contains(&param.accession.1) {
                return Err(CvParamsValidationError::Duplication(
                    param.accession.0.clone(),
                    param.accession.1,
                )
                .into());
            } else {
                visited_ids.insert(param.accession.1);
            }
        }

        Ok(matching_cv_params)
    }

    /// Checks if the given cvParams apply to this rule
    ///
    /// # Arguments:
    /// * `cv_params` - Iterator over cvParams to validate
    /// * `strict` - Missing SHOULD returns error
    ///
    pub fn validate<'a, I>(
        &'static self,
        cv_params: I,
        strict: bool,
    ) -> Result<Vec<&'a CvParam>, ValidationError>
    where
        I: Iterator<Item = &'a CvParam>,
    {
        match self.occurence {
            CvParamOccurence::MustOnce => self.validate_must_once(cv_params),
            CvParamOccurence::MustOnceOrMany => self.validate_must_once_or_many(cv_params),
            CvParamOccurence::MayOnce => self.validate_may_once(cv_params),
            CvParamOccurence::MayOnceOrMany => self.validate_may_once_or_many(cv_params),
            CvParamOccurence::ShouldOnceOrMany => {
                if strict {
                    self.validate_should_once_or_many(cv_params)
                } else {
                    Ok(vec![])
                }
            }
        }
    }
}

impl Display for CvParamRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let child = if self.supplies_children {
            "Child of "
        } else {
            ""
        };
        match self.occurence {
            CvParamOccurence::MustOnce => write!(
                f,
                "{child}{}:{} must be supplied once.",
                self.cv_name, self.id
            ),
            CvParamOccurence::MustOnceOrMany => write!(
                f,
                "{child}{}:{} must be supplied at least once.",
                self.cv_name, self.id
            ),
            CvParamOccurence::MayOnce => write!(
                f,
                "{child}{}:{} can be supplied only once.",
                self.cv_name, self.id
            ),
            CvParamOccurence::MayOnceOrMany => write!(
                f,
                "{child}{}:{} can be supplied once or many times.",
                self.cv_name, self.id
            ),
            CvParamOccurence::ShouldOnceOrMany => write!(
                f,
                "{child}{}:{} should be supplied once or many times.",
                self.cv_name, self.id
            ),
        }
    }
}

/// Trait to deal with validation of cvParams
///
pub trait HasCvParams {
    /// CV rules applying to a Element
    ///
    const CV_PARAM_RULES: &[CvParamRule];

    /// Returns a reference to the cvParams of the element.
    ///
    fn cv_params(&self) -> impl Iterator<Item = &CvParam>;

    /// Returnes the rules for cvParams
    ///
    fn cv_param_rules() -> &'static [CvParamRule] {
        Self::CV_PARAM_RULES
    }

    /// Returns a reference to the cvParam with the given accession.
    /// Some accessions can occur multiple times,
    /// therefore a vector is returned.
    ///
    /// # Arguments
    /// * `accession` - The accession of the cvParam to be retrieved.
    ///
    fn cv_param_by_accession(
        &self,
        cv_name: &str,
        term_id: usize,
    ) -> Result<Vec<&CvParam>, CvError> {
        Ok(self
            .cv_params()
            .filter(|cv_param| cv_param.accession.0 == cv_name && cv_param.accession.1 == term_id)
            .collect())
    }

    /// Validate the given cvParams list
    ///
    /// # Arguments
    /// * `strict` - If true, missing SHOULD terms will return an error
    ///
    fn validate_cv_params(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        for param in self.cv_params() {
            param.validate(version, strict)?;
        }
        for rule in Self::cv_param_rules() {
            rule.validate(self.cv_params(), strict)?;
        }
        Ok(())
    }
}

/// This macro generates the implementation of the `HasCvParams` trait for the given struct which has mutliple cvParams
///
/// # Arguments
/// * `$name` - The name of the struct for which the implementation is being generated.
/// * `$cv_params` - Struct field which contains cvParams
/// * `$rules` - Array of CV rules to apply to the cvParams of an element
///
#[macro_export]
macro_rules! has_cv_params {
    // With rules
    ($name:ty, $cv_params:ident, [$($rules:expr),* $(,)?]) => {
        use $crate::elements::has_cv_params::{HasCvParams, CvParamRule, CvParamOccurence};

        impl HasCvParams for $name {
            const CV_PARAM_RULES: &[CvParamRule] =  &[$($rules),*];

            fn cv_params(&self) -> impl Iterator<Item = &CvParam> {
                self.$cv_params.iter()
            }
        }
    };

    // Without rules
    ($name:ty, $cv_params:ident) => {
        use $crate::elements::has_cv_params::{CvParamRule, HasCvParams};

        impl HasCvParams for $name {
            const CV_PARAM_RULES: &[CvParamRule] = &[];

            fn cv_params(&self) -> impl Iterator<Item = &CvParam> {
                self.$cv_params.iter()
            }
        }
    };
}

/// This macro generates the implementation of the `HasCvParams` trait for the given struct which has one cvParam
///
/// # Arguments
/// * `$name` - The name of the struct for which the implementation is being generated.
/// * `$cv_param` - Struct field which contains the cvParam
/// * `$rules` - Array of CV rules to apply to the cvParams of an element
///
#[macro_export]
macro_rules! has_cv_param {

    // With rules
    ($name:ty, $cv_param:ident, [$($rules:expr),* $(,)?]) => {
        use $crate::elements::has_cv_params::{HasCvParams, CvParamRule, CvParamOccurence};

        impl HasCvParams for $name {
            const CV_PARAM_RULES: &[CvParamRule] =  &[$($rules),*];

            fn cv_params(&self) -> impl Iterator<Item = &CvParam> {
                std::iter::once(&self.$cv_param)
            }
        }
    };
}

/// This macro generates the implementation of the `HasCvParams` trait for the given struct which has one optional cvParam
///
/// # Arguments
/// * `$name` - The name of the struct for which the implementation is being generated.
/// * `$opt_cv_param` - Struct field which contains the optional cvParam
/// * `$rules` - Array of CV rules to apply to the cvParams of an element
///
#[macro_export]
macro_rules! has_opt_cv_param {
    ($name:ty, $opt_cv_param:ident, [$($rules:expr),* $(,)?]) => {
        use $crate::elements::has_cv_params::{HasCvParams, CvParamRule, CvParamOccurence};

        impl HasCvParams for $name {
            const CV_PARAM_RULES: &[CvParamRule] =  &[$($rules),*];

            fn cv_params(&self) -> impl Iterator<Item = &CvParam> {
                let iter: Box<dyn Iterator<Item = &CvParam>> = match self.$opt_cv_param.as_ref() {
                    Some(cv_param) => Box::new(std::iter::once(cv_param)),
                    None => Box::new(std::iter::empty()),
                };

                iter
            }
        }
    };
}

use crate::{elements::attributes::semver::SemVer, error::ValidationError};

pub trait IsElement {
    /// Validates the mzML element
    ///
    /// # Arguments
    /// * `version`- Document version
    /// * `strict` - If true, perform strict validation
    ///
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError>;
}

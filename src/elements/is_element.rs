use crate::error::ValidationError;

pub trait IsElement {
    /// Validates the mzML element
    ///
    /// # Arguments
    /// * `strict` - If true, perform strict validation
    ///
    fn validate(&self, strict: bool) -> Result<(), ValidationError>;
}

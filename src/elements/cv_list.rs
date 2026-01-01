use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::{cv::Cv, is_element::IsElement, is_list::IsList};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "cv")]
    pub cv: Vec<Cv>,
}

impl IsElement for CvList {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        for cv in &self.cv {
            cv.validate(strict)?;
        }
        Ok(())
    }
}

impl IsList<'_, Cv> for CvList {
    fn list_items(&self) -> &[Cv] {
        &self.cv
    }
}

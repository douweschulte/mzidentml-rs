use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, parent::Parent,
        user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
    #[serde(rename = "Parent")]
    pub parent: Option<Parent>,
}

impl IsElement for Organization {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("Organization", "id"));
        }
        self.validate_cv_params(version, strict)?;
        for param in &self.user_params {
            param.validate(version, strict)?;
        }
        if let Some(parent) = &self.parent {
            parent.validate(version, strict)?;
        }
        Ok(())
    }
}

has_cv_params!(
    Organization,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1000588,
            occurence: CvParamOccurence::ShouldOnceOrMany,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1000587,
            occurence: CvParamOccurence::ShouldOnceOrMany,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1000589,
            occurence: CvParamOccurence::ShouldOnceOrMany,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1000586,
            occurence: CvParamOccurence::ShouldOnceOrMany,
            supplies_children: false,
        },
    ]
);

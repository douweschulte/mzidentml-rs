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
    const ELEMENT_TAG: &str = "Organization";

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
        self.validate_cv_params(version, strict, element_path)?;
        Self::validate_elements(version, strict, element_path, self.user_params.iter())?;

        if let Some(parent) = &self.parent {
            parent.validate(version, strict, element_path, None)?;
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

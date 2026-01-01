use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        affiliation::Affiliation, cv_param::CvParam, is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "@lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "@midInitials")]
    pub mid_initials: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
    #[serde(default, rename = "Affiliation")]
    pub affiliations: Vec<Affiliation>,
}

impl IsElement for Person {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.name.is_none() && (self.first_name.is_none() || self.last_name.is_none()) {
            return Err(ValidationError::EmptyAttribute("Person", "name"));
        }
        for user_param in &self.user_params {
            user_param.validate(strict)?;
        }
        for affiliation in &self.affiliations {
            affiliation.validate(strict)?;
        }
        self.validate_cv_params(strict)?;
        Ok(())
    }
}

has_cv_params!(
    Person,
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
        }
    ]
);

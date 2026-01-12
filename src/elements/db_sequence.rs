use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement, seq::Seq,
        user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbSequence {
    #[serde(rename = "@accession")]
    pub accession: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@searchDatabase_ref")]
    pub search_database_ref: String,

    #[serde(rename = "@length")]
    pub length: Option<usize>,
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "Seq")]
    pub sequence: Seq,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for DbSequence {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.accession.is_empty() {
            return Err(ValidationError::EmptyAttribute("DbSequence", "accession"));
        }

        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("DbSequence", "id"));
        }

        if self.search_database_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "DbSequence",
                "searchDatabase_ref",
            ));
        }

        self.validate_cv_params(version, strict)?;

        for user_param in &self.user_params {
            user_param.validate(version, strict)?;
        }
        self.sequence.validate(version, strict)?;

        Ok(())
    }
}

has_cv_params!(
    DbSequence,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1001342,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001089,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        }
    ]
);

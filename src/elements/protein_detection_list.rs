use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement,
        protein_ambiguity_group::ProteinAmbiguityGroup, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProteinDetectionList {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(default, rename = "ProteinAmbiguityGroup")]
    pub protein_ambiguity_groups: Vec<ProteinAmbiguityGroup>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for ProteinDetectionList {
    const ELEMENT_TAG: &str = "ProteinDetectionList";

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

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.protein_ambiguity_groups.iter(),
        )?;

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())
    }
}

has_cv_params!(
    ProteinDetectionList,
    cv_params,
    [
        CvParamRule {
            cv_name: "MS",
            id: 1002404,
            occurence: CvParamOccurence::MustOnce,
            supplies_children: false,
        },
        CvParamRule {
            cv_name: "MS",
            id: 1001184,
            occurence: CvParamOccurence::MayOnceOrMany,
            supplies_children: true,
        },
    ]
);

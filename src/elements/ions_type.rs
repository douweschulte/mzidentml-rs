use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, cv_param::CvParam, fragment_array::FragmentArray,
        is_element::IsElement, user_param::UserParam,
    },
    error::ValidationError,
    has_cv_params,
    parsing::opt_space_separated_vec_parsing,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IonType {
    #[serde(rename = "@index", with = "opt_space_separated_vec_parsing")]
    pub index: Option<Vec<usize>>,
    #[serde(rename = "FragmentArray")]
    pub fragment_arrays: Vec<FragmentArray>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for IonType {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        for array in self.fragment_arrays.iter() {
            array.validate(version, strict)?;
        }

        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "IonType", "cvParam",
            ));
        }

        self.validate_cv_params(version, strict)?;

        for param in self.user_params.iter() {
            param.validate(version, strict)?;
        }

        Ok(())
    }
}

has_cv_params!(
    IonType,
    cv_params,
    [CvParamRule {
        cv_name: "MS",
        id: 1001221,
        occurence: CvParamOccurence::MayOnceOrMany,
        supplies_children: true,
    },]
);

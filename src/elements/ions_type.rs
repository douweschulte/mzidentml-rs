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
    const ELEMENT_TAG: &str = "IonType";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(version, strict, element_path, self.fragment_arrays.iter())?;

        if self.cv_params.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "cvParam",
            ));
        }

        self.validate_cv_params(version, strict, element_path)?;

        Self::validate_elements(version, strict, element_path, self.user_params.iter())
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

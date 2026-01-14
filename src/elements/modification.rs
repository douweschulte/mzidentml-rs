use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, cv_param::CvParam, is_element::IsElement},
    error::ValidationError,
    has_cv_params,
    parsing::opt_space_separated_vec_parsing,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modification {
    #[serde(rename = "@avgMassDelta")]
    pub avg_mass_deltaa: Option<f64>,
    #[serde(rename = "@location")]
    pub location: Option<usize>,
    #[serde(rename = "@monoisotopicMassDel")]
    pub monoisotopic_mass_del: Option<f64>,
    #[serde(
        default,
        rename = "@residues",
        with = "opt_space_separated_vec_parsing"
    )]
    pub residues: Option<Vec<char>>,
    #[serde(rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Modification {
    const ELEMENT_TAG: &str = "Modification";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        self.validate_cv_params(version, strict, element_path)
    }
}

has_cv_params!(Modification, cv_params);

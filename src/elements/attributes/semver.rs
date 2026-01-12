use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// Simple struct to represnet version of format `major.minor.patch` element.
/// This follows mzIdentML doc's `x.y.z`. Much simpler then the SemVer crate but should the job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SemVer {
    /// Returns major version
    ///
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Returns minor version
    ///
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns patch version
    ///
    pub fn patch(&self) -> u32 {
        self.patch
    }
}

impl TryFrom<String> for SemVer {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() != 3 {
            return Err(Self::Error::InvalidVersion(value));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| Self::Error::InvalidVersion(value.clone()))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| Self::Error::InvalidVersion(value.clone()))?;
        let patch = parts[2]
            .parse::<u32>()
            .map_err(|_| Self::Error::InvalidVersion(value.clone()))?;

        Ok(SemVer {
            major,
            minor,
            patch,
        })
    }
}

impl From<SemVer> for String {
    fn from(value: SemVer) -> String {
        format!("{}.{}.{}", value.major(), value.minor(), value.patch())
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

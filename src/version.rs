use std::fmt;

mod errors;
use errors::VersionParseError;

mod rpm_ver;
use rpm_ver::RPMVersion;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    epoch: Option<u64>,
    version: RPMVersion,
    release: u64,
}

impl Version {
    pub fn new(
        epoch: impl Into<Option<u64>>,
        version: impl Into<RPMVersion>,
        release: u64,
    ) -> Self {
        Self {
            epoch: epoch.into(),
            version: version.into(),
            release,
        }
    }

    pub fn from_string(version: impl Into<String>) -> Result<Self, VersionParseError> {
        let (epoch, version) = extract_epoch(&version.into())?;
        let (version, release) = extract_release(&version)?;
        Ok(Self::new(epoch, version, release))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.epoch {
            Some(epoch) => write!(f, "{}:{}-{}", epoch, self.version, self.release),
            None => write!(f, "{}-{}", self.version, self.release),
        }
    }
}

fn extract_epoch(version: &String) -> Result<(Option<u64>, String), VersionParseError> {
    match version.split_once(':') {
        Some((epoch, remainder)) => match epoch.parse::<u64>() {
            Ok(epoch) => Ok((Some(epoch), String::from(remainder))),
            Err(_) => Err(VersionParseError::InvalidEpoch),
        },
        None => Ok((None, String::from(version))),
    }
}

fn extract_release(version: &str) -> Result<(String, u64), VersionParseError> {
    match version.split_once('-') {
        Some((remainder, release)) => match release.parse::<u64>() {
            Ok(release) => Ok((String::from(remainder), release)),
            Err(_) => Err(VersionParseError::InvalidRelease),
        },
        None => Err(VersionParseError::NoReleaseSpecified),
    }
}

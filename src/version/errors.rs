use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum VersionParseError {
    InvalidEpoch,
    InvalidRelease,
    NoReleaseSpecified,
}

impl VersionParseError {
    pub fn to_string(&self) -> &str {
        match self {
            Self::InvalidEpoch => "invalid epoch",
            Self::InvalidRelease => "invalid release",
            Self::NoReleaseSpecified => "no release specified",
        }
    }
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

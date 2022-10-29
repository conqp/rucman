use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SigLevel {
    Never,
    Optional,
    Required,
    TrustedOnly,
    TrustAll,
}

impl FromStr for SigLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Never" => Ok(Self::Never),
            "Optional" => Ok(Self::Optional),
            "Required" => Ok(Self::Required),
            "TrustedOnly" => Ok(Self::TrustedOnly),
            "TrustAll" => Ok(Self::TrustAll),
            _ => Err("invalid SigLevel".to_string()),
        }
    }
}

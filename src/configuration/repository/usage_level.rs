use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsageLevel {
    Sync,
    Search,
    Install,
    Upgrade,
    All,
}

impl FromStr for UsageLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sync" => Ok(Self::Sync),
            "Search" => Ok(Self::Search),
            "Install" => Ok(Self::Install),
            "Upgrade" => Ok(Self::Upgrade),
            "All" => Ok(Self::All),
            _ => Err("invalid usage level".to_string()),
        }
    }
}

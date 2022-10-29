use url::Url;

pub(crate) mod sig_level;
use sig_level::SigLevel;

pub(crate) mod usage_level;
use usage_level::UsageLevel;

pub trait Repository {
    fn server(&self) -> &Url;
    fn sig_level(&self) -> Vec<SigLevel>;
    fn usage(&self) -> Vec<UsageLevel>;
}

#[derive(Debug)]
pub(crate) struct PacmanRepository {
    server: Url,
    sig_level: Vec<SigLevel>,
    usage: Vec<UsageLevel>,
}

impl Repository for PacmanRepository {
    fn server(&self) -> &Url {
        &self.server
    }

    fn sig_level(&self) -> Vec<SigLevel> {
        self.sig_level.clone()
    }

    fn usage(&self) -> Vec<UsageLevel> {
        self.usage.clone()
    }
}

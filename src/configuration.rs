use std::collections::HashMap;

pub(crate) mod options;
use options::{Options, PacmanConfOptions};

pub(crate) mod repository;
use repository::{PacmanRepository, Repository};

pub trait Configuration<O: Options, R: Repository> {
    fn options(&self) -> &O;
    fn repositories(&self) -> HashMap<&str, &R>;
}

#[derive(Debug)]
struct PacmanConf {
    options: PacmanConfOptions,
    repositories: HashMap<String, PacmanRepository>,
}

impl Configuration<PacmanConfOptions, PacmanRepository> for PacmanConf {
    fn options(&self) -> &PacmanConfOptions {
        &self.options
    }

    fn repositories(&self) -> HashMap<&str, &PacmanRepository> {
        let iter = self
            .repositories
            .iter()
            .map(|(key, value)| (key.as_str(), value));
        HashMap::from_iter(iter)
    }
}

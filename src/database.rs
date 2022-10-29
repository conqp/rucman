use std::collections::{HashMap, HashSet};

pub enum Status {
    Valid,
    Invalid,
    Exists,
    Missing,
    Local,
    PkgCache,
    GrpCache,
}

// TODO: Mock structs
struct Package {}
enum SigLevel {}
enum Usage {}

pub struct Database {
    tree_name: String,
    path: String,
    pkg_cache: HashMap<String, Package>,
    grp_cache: Vec<String>,
    servers: Vec<String>,
    status: HashSet<Status>,
    sig_level: Vec<SigLevel>,
    usage: Vec<Usage>,
}

impl Database {
    fn validate(&self) -> bool {
        // TODO: implement
        false
    }

    fn populate(&mut self) -> bool {
        // TODO: implement
        false
    }

    fn unregister(&mut self) {}
}

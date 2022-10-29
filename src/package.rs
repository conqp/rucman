use crate::handle::Architecture;
use crate::{File, Version};
use md5::Digest;
use sha2::Sha256;
use std::time::SystemTime;

pub struct Package {
    name_hash: u64,
    filename: String,
    base: String,
    name: String,
    version: Version,
    desc: String,
    url: String,
    packager: String,
    md5sum: Digest,
    sha256sum: Sha256,
    b64sig: String,
    arch: Architecture,
    build_date: SystemTime,
    install_date: SystemTime,
    size: usize,
    isize: usize,
    download_size: usize,
    // handle: Handle,
    licenses: Vec<String>,
    replaces: Vec<String>,
    groups: Vec<String>,
    backup: Vec<String>,
    depends: Vec<String>,
    optdepends: Vec<String>,
    makedepends: Vec<String>,
    conflicts: Vec<String>,
    provides: Vec<String>,
    removes: Vec<String>,
    // oldpkg: Package,
    files: Vec<File>,
    // TODO: continue to implement
}

use std::path::Path;

use log::Level;
use url::Url;

// Mockup definitions
pub struct Context;
pub trait Event {}
pub enum DownloadError {}
pub trait Question {}
pub enum ProgressType {}
pub struct Package {}
pub enum Architecture {}
pub enum SignatureLevel {}

pub trait Callback {
    fn log_callback(&self, context: &Context, level: &Level, message: &str);
    fn download_callback(&self, context: &Context, filename: impl AsRef<Path>, event: &impl Event);
    fn fetch_callback(
        &self,
        context: &Context,
        url: impl AsRef<Url>,
        directory: impl AsRef<Path>,
        force: bool,
    ) -> Result<bool, DownloadError>;
    fn event_callback(&self, context: &Context, event: &impl Event);
    fn question_callback(&self, context: &Context, question: &impl Question);
    fn progress_callback(
        &self,
        context: &Context,
        progress_type: ProgressType,
        package: Option<&str>,
        percent: u8,
        total: usize,
        processed: usize,
    );
}

pub trait FilesystemPaths {
    fn root(&self) -> &Path;
    fn dbpath(&self) -> &Path;
    fn logfile(&self) -> &Path;
    fn lockfile(&self) -> &Path;
    fn gpgdir(&self) -> &Path;
    fn cache_dirs(&self) -> Vec<&Path>;
    fn hook_dirs(&self) -> Vec<&Path>;
    fn overwrite_files(&self) -> Vec<&Path>;
}

pub trait PackageLists {
    fn do_not_upgrade(&self) -> Vec<Package>;
    fn do_not_extract(&self) -> Vec<&Path>;
    fn ignored_packages(&self) -> Vec<&str>;
    fn ignored_groups(&self) -> Vec<&str>;
    fn assume_installed(&self) -> Vec<&str>;
}

pub trait Options {
    fn download_timeout(&self) -> Option<u8>;
    fn parallel_downloads(&self) -> Option<u8>;
    fn known_keys(&self) -> Vec<&str>;
    fn architectures(&self) -> Vec<Architecture>;
    fn use_syslog(&self) -> bool;
    fn check_space(&self) -> bool;
    fn database_suffix(&self) -> &str;
    fn signature_level(&self) -> SignatureLevel;
    fn local_files_signature_level(&self) -> SignatureLevel;
    fn remote_files_signature_level(&self) -> SignatureLevel;
}

pub trait Handle: Callback + FilesystemPaths + PackageLists + Options {}

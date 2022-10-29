const ROOT_DIR: &str = "/";
const DB_PATH: &str = "/var/lib/pacman/";
const CACHE_DIR: &str = "/var/cache/pacman/pkg/";
const LOG_FILE: &str = "/var/log/pacman.log";
const GPG_DIR: &str = "/etc/pacman.d/gnupg/";
const HOOK_DIR: &str = "/etc/pacman.d/hooks/";

pub trait Options {
    fn root_dir(&self) -> &str;
    fn db_path(&self) -> &str;
    fn cache_dir(&self) -> &str;
    fn hook_dir(&self) -> &str;
    fn gpg_dir(&self) -> &str;
    fn log_file(&self) -> &str;
    fn hold_pkg(&self) -> Vec<&str>;
    fn ignore_pkg(&self) -> Vec<&str>;
    fn ignore_group(&self) -> Vec<&str>;
    fn architectures(&self) -> Vec<&str>;
    fn transfer_command(&self) -> Option<&str>;
    fn no_upgrade(&self) -> Vec<&str>;
    fn no_extract(&self) -> Vec<&str>;
    fn clean_method(&self) -> Vec<&str>;
    fn sig_level(&self) -> Vec<&str>;
    fn local_file_sig_level(&self) -> Vec<&str>;
    fn remote_file_sig_level(&self) -> Vec<&str>;
    fn use_syslog(&self) -> bool;
    fn color(&self) -> bool;
    fn no_progress_bar(&self) -> bool;
    fn check_space(&self) -> bool;
    fn verbose_pkg_lists(&self) -> bool;
    fn disable_download_timeout(&self) -> bool;
    fn parallel_downloads(&self) -> u8;
}

#[derive(Debug)]
pub(crate) struct PacmanConfOptions {
    // TODO: handle Include=
    root_dir: Option<String>,
    db_path: Option<String>,
    cache_dir: Option<String>,
    hook_dir: Option<String>,
    gpg_dir: Option<String>,
    log_file: Option<String>,
    hold_pkg: Vec<String>,
    ignore_pkg: Vec<String>,
    ignore_group: Vec<String>,
    architectures: Vec<String>,
    transfer_command: Option<String>,
    no_upgrade: Vec<String>,
    no_extract: Vec<String>,
    clean_method: Vec<String>,
    sig_level: Vec<String>,
    local_file_sig_level: Vec<String>,
    remote_file_sig_level: Vec<String>,
    use_syslog: bool,
    color: bool,
    no_progress_bar: bool,
    check_space: bool,
    verbose_pkg_lists: bool,
    disable_download_timeout: bool,
    parallel_downloads: u8,
}

impl PacmanConfOptions {
    //pub fn from_file(filename: impl AsRef<Path>) -> Self {
    //    //TODO implement
    //}
}

impl Options for PacmanConfOptions {
    fn root_dir(&self) -> &str {
        match &self.root_dir {
            Some(root_dir) => root_dir,
            None => ROOT_DIR,
        }
    }

    fn db_path(&self) -> &str {
        match &self.db_path {
            Some(db_path) => db_path,
            None => DB_PATH,
        }
    }

    fn cache_dir(&self) -> &str {
        match &self.cache_dir {
            Some(cache_dir) => cache_dir,
            None => CACHE_DIR,
        }
    }

    fn hook_dir(&self) -> &str {
        match &self.hook_dir {
            Some(hook_dir) => hook_dir,
            None => HOOK_DIR,
        }
    }

    fn gpg_dir(&self) -> &str {
        match &self.gpg_dir {
            Some(gpg_dir) => gpg_dir,
            None => GPG_DIR,
        }
    }

    fn log_file(&self) -> &str {
        match &self.log_file {
            Some(log_file) => log_file,
            None => LOG_FILE,
        }
    }

    fn hold_pkg(&self) -> Vec<&str> {
        self.hold_pkg.iter().map(|string| string.as_str()).collect()
    }

    fn ignore_pkg(&self) -> Vec<&str> {
        self.ignore_pkg
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn ignore_group(&self) -> Vec<&str> {
        self.ignore_group
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn architectures(&self) -> Vec<&str> {
        self.architectures
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn transfer_command(&self) -> Option<&str> {
        Some(self.transfer_command.as_ref()?)
    }

    fn no_upgrade(&self) -> Vec<&str> {
        self.no_upgrade
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn no_extract(&self) -> Vec<&str> {
        self.no_extract
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn clean_method(&self) -> Vec<&str> {
        self.clean_method
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn sig_level(&self) -> Vec<&str> {
        self.sig_level
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn local_file_sig_level(&self) -> Vec<&str> {
        self.local_file_sig_level
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn remote_file_sig_level(&self) -> Vec<&str> {
        self.remote_file_sig_level
            .iter()
            .map(|string| string.as_str())
            .collect()
    }

    fn use_syslog(&self) -> bool {
        self.use_syslog
    }

    fn color(&self) -> bool {
        self.color
    }

    fn no_progress_bar(&self) -> bool {
        self.no_progress_bar
    }

    fn check_space(&self) -> bool {
        self.check_space
    }

    fn verbose_pkg_lists(&self) -> bool {
        self.verbose_pkg_lists
    }

    fn disable_download_timeout(&self) -> bool {
        self.disable_download_timeout
    }

    fn parallel_downloads(&self) -> u8 {
        self.parallel_downloads
    }
}

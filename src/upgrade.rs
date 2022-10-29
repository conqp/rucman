use crate::handle::Handle;
use crate::package::Package;

pub enum PackageUpgradeError {
    // TODO: implement
    OhNoSomethingWentWrong,
}

pub fn add_package(handle: &impl Handle, package: &Package) -> Result<bool, PackageUpgradeError> {
    Err(PackageUpgradeError::OhNoSomethingWentWrong)
}

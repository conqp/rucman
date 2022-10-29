pub use equal_versions::EQUAL;
pub use greater_versions::GREATER_THAN;
pub use lesser_versions::LESS_THAN;
pub use parsed_versions::VERSIONS;
use rucman::Version;
pub use valid_versions::VALID_VERSIONS;

mod equal_versions;
mod greater_versions;
mod lesser_versions;
mod parsed_versions;
mod valid_versions;

pub fn load_version_pair((lhs, rhs): (&str, &str)) -> (Version, Version) {
    (
        Version::from_string(lhs).unwrap(),
        Version::from_string(rhs).unwrap(),
    )
}

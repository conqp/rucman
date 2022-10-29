use std::cmp::Ordering;

use common::{load_version_pair, EQUAL, GREATER_THAN, LESS_THAN, VALID_VERSIONS, VERSIONS};
use rucman::Version;

mod common;

#[test]
fn version_parsing() {
    for (string, version) in VERSIONS.iter() {
        assert_eq!(*version, Version::from_string(*string).unwrap());
    }
}

#[test]
fn version_deserialization() {
    for string in VALID_VERSIONS {
        assert!(Version::from_string(string).is_ok());
    }
}

#[test]
fn version_serialization() {
    for string in VALID_VERSIONS {
        assert_eq!(string, Version::from_string(string).unwrap().to_string());
    }

    for (string, version) in VERSIONS.iter() {
        assert_eq!(*string, version.to_string());
    }
}

#[test]
fn version_comparison() {
    for (lhs, rhs) in EQUAL.map(load_version_pair) {
        assert_eq!(lhs, rhs);
        assert_eq!(Ordering::Equal, lhs.cmp(&rhs))
    }

    for (lhs, rhs) in GREATER_THAN.map(load_version_pair) {
        assert!(lhs > rhs);
        assert_eq!(Ordering::Greater, lhs.cmp(&rhs))
    }

    for (lhs, rhs) in LESS_THAN.map(load_version_pair) {
        assert!(lhs < rhs);
        assert_eq!(Ordering::Less, lhs.cmp(&rhs))
    }
}

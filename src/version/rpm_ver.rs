use std::cmp::Ordering;
use std::fmt;

mod segment;
use segment::Segment;

const SEGMENT_SEPARATOR: char = '.';

#[derive(Debug, Eq, PartialEq)]
pub struct RPMVersion(String);

impl RPMVersion {
    fn normalize(&self) -> String {
        self.0
            .chars()
            .map(|chr| {
                if chr.is_alphanumeric() {
                    chr
                } else {
                    SEGMENT_SEPARATOR
                }
            })
            .collect()
    }

    fn segments(&self) -> Vec<Segment> {
        self.normalize()
            .split(SEGMENT_SEPARATOR)
            .map(Segment::from)
            .collect()
    }
}

impl fmt::Display for RPMVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RPMVersion {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for RPMVersion {
    fn from(s: &str) -> Self {
        Self(String::from(s))
    }
}

impl Ord for RPMVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }

        let lhs = &self.segments();
        let rhs = &other.segments();

        for (l_segment, r_segment) in lhs.iter().zip(rhs.iter()) {
            match l_segment.cmp(r_segment) {
                Ordering::Equal => {
                    continue;
                }
                ordering => {
                    return ordering;
                }
            }
        }

        lhs.len().cmp(&rhs.len())
    }
}

impl PartialOrd for RPMVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

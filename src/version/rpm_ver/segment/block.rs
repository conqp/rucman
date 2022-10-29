use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Block(String);

impl Block {
    pub fn is_numeric(&self) -> bool {
        self.0.chars().all(|chr| chr.is_ascii_digit())
    }
}

impl From<String> for Block {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }

        let l_is_number = self.0.chars().all(|chr| chr.is_ascii_digit());
        let r_is_number = other.0.chars().all(|chr| chr.is_ascii_digit());

        if l_is_number && r_is_number {
            compare_alpha(&self.0, &other.0)
        } else if l_is_number && !r_is_number {
            Ordering::Greater
        } else if !l_is_number && r_is_number {
            Ordering::Less
        } else {
            self.0.cmp(&other.0)
        }
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_alpha(lhs: &str, rhs: &str) -> Ordering {
    if lhs == rhs {
        return Ordering::Equal;
    }

    let lhs = lhs.trim_start_matches('0');
    let rhs = rhs.trim_start_matches('0');

    match lhs.len().cmp(&rhs.len()) {
        Ordering::Equal => lhs.cmp(rhs),
        ordering => ordering,
    }
}

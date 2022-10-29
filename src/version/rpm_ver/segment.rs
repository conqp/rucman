use std::cmp::Ordering;

mod block;
use block::Block;

#[derive(Debug, Eq, PartialEq)]
pub struct Segment(String);

impl Segment {
    fn blocks(&self) -> Vec<Block> {
        let mut result = Vec::new();
        let mut block = String::new();

        for chr in self.0.chars() {
            match block.chars().next() {
                Some(current) => {
                    if same_type(&chr, &current) {
                        block.push(chr);
                    } else {
                        result.push(Block::from(block));
                        block = String::new();
                        block.push(chr);
                    }
                }
                None => {
                    block.push(chr);
                }
            }
        }

        if !block.is_empty() {
            result.push(Block::from(block));
        }

        result
    }
}

impl From<&str> for Segment {
    fn from(s: &str) -> Self {
        Self(String::from(s))
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }

        let l_blocks = self.blocks();
        let r_blocks = other.blocks();
        let mut last: usize = 0;

        for (index, (l_block, r_block)) in l_blocks.iter().zip(r_blocks.iter()).enumerate() {
            last = index;

            match l_block.cmp(r_block) {
                Ordering::Equal => {
                    continue;
                }
                ordering => {
                    return ordering;
                }
            }
        }

        match l_blocks.get(last + 1) {
            Some(block) => {
                if block.is_numeric() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            None => match r_blocks.get(last + 1) {
                Some(block) => {
                    if block.is_numeric() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                None => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn same_type(lhs: &char, rhs: &char) -> bool {
    lhs.is_ascii_digit() == rhs.is_ascii_digit()
}

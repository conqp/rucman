use librucman::version::Version;
use std::cmp::Ordering;

mod args;
use args::{parse, Args};

fn main() {
    let args = parse();

    match Version::from_string(args.lhs()) {
        Ok(lhs) => match Version::from_string(args.rhs()) {
            Ok(rhs) => {
                println!(
                    "{}",
                    match lhs.cmp(&rhs) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    }
                );
            }
            Err(err) => {
                eprintln!("Invalid version: {} ({})", args.rhs(), err);
            }
        },
        Err(err) => {
            eprintln!("Invalid version: {} ({})", args.lhs(), err);
        }
    }
}

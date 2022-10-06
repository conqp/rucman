use clap::Parser;
use librucman::Version;
use std::cmp::Ordering;

use rucman::cmpver::Args;

fn main() {
    let args = Args::parse();

    match Version::from_string(&args.lhs) {
        Ok(lhs) => match Version::from_string(&args.rhs) {
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
                eprintln!("Invalid version: {} ({})", args.rhs, err);
            }
        },
        Err(err) => {
            eprintln!("Invalid version: {} ({})", args.lhs, err);
        }
    }
}

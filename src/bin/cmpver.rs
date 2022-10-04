use clap::Parser;
use librucman::version::Version;
use std::cmp::Ordering;

#[derive(Parser)]
#[clap(name = "cmpver")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "1.0.0")]
#[clap(about = "Compare rucman package versions.", long_about = None)]
struct Args {
    #[clap(value_parser)]
    pub lhs: String,

    #[clap(value_parser)]
    pub rhs: String,
}

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

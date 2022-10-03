use clap::Parser;

#[derive(Parser)]
#[clap(name = "cmpver")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "1.0.0")]
#[clap(about = "Compare rucman package versions.", long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    pub lhs: String,

    #[clap(value_parser)]
    pub rhs: String,
}

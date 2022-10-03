use clap::Parser;

pub trait Args {
    fn lhs(&self) -> &String;
    fn rhs(&self) -> &String;
}

#[derive(Parser)]
#[clap(name = "cmpver")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "1.0.0")]
#[clap(about = "Compare rucman package versions.", long_about = None)]
struct ArgumentParser {
    #[clap(value_parser)]
    lhs: String,

    #[clap(value_parser)]
    rhs: String,
}

pub fn parse() -> impl Args {
    ArgumentParser::parse()
}

impl Args for ArgumentParser {
    fn lhs(&self) -> &String {
        &self.lhs
    }

    fn rhs(&self) -> &String {
        &self.rhs
    }
}

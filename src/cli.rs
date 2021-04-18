use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Lazy Mechanic")]
pub struct Cli {
    /// Config path
    #[clap(long, short)]
    pub config: Option<String>,
}

impl Cli {
    pub fn parse_args() -> Cli {
        Cli::parse()
    }
}

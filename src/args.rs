use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Optional seed to use in all random operations
    #[arg(short, long, default_value_t = rand::random())]
    pub seed: u64,
}

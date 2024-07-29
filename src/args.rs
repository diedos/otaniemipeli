use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Test
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

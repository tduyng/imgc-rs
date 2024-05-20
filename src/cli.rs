use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(short, long)]
    pub dir: String,

    #[clap(short, long)]
    pub output: Option<String>,
}

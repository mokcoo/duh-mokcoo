

#[derive(Debug, clap::Parser)]
#[clap(name = "duam",  version)]
#[clap(override_usage = "duam [FLAGS] [OPTIONS] [SUBCOMMAND] [INPUT]...")]
pub struct Arg {
    #[clap(short = 'x', long)]
    pub stay_on_filesystem: bool
}
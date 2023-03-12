use clap::{ArgAction, Parser};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(action = ArgAction::Set, short, long, default_value_t  = true)]
    pub udp: bool,

    #[clap(action = ArgAction::Set, short, long, default_value_t  = false)]
    pub tcp: bool,

    #[clap(short, long, default_value_t = 53)]
    pub port: u32,
}

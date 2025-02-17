use clap::Parser;
use clio::Input;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(value_parser, default_value = "-")]
    pub input: Input,

    #[arg(long, short)]
    pub cfg_file: Option<PathBuf>,

    #[arg(long)]
    pub color_always: bool,

    #[arg(long)]
    pub show_cfg_path: bool,
}

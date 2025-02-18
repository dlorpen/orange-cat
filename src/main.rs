mod cli;
mod config;

use std::io::{self, BufRead, BufReader, Write};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use config::{load_config, set_color_always, show_cfg_path};

fn main() -> Result<()> {
    config::setup_panic();

    let args = cli::Args::parse();

    if args.show_cfg_path {
        show_cfg_path();
        return Ok(());
    }

    if args.color_always {
        set_color_always();
    }

    let config = load_config(args.cfg_file);

    let matching_fns: Vec<_> = config.get_matching_rules();

    let mut output = io::stdout().lock();

    for mut input in args.input {
        let mut input = BufReader::new(input.lock());

        let mut line = String::new();
        while input.read_line(&mut line)? > 0 {
            if matching_fns.iter().all(|func| !func(&line, &mut output)) {
                write!(output, "{}", line.white())?;
            }
            output.flush()?;
            line.clear();
        }
    }

    Ok(())
}

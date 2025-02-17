use colored::Colorize;
use panic::setup_panic;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

pub fn load_config(config_path: Option<PathBuf>) -> Config {
    match config_path {
        Some(config_file_path) => {
            confy::load_path(config_file_path).expect("Error loading specified config file")
        }
        _ => confy::load(env!("CARGO_PKG_NAME"), None).expect("Error loading config"),
    }
}

pub fn set_color_always() {
    colored::control::set_override(true);
}

pub fn show_cfg_path() {
    println!(
        "{}",
        confy::get_configuration_file_path(env!("CARGO_PKG_NAME"), None)
            .expect("Error getting config file path")
            .display()
    );
}

pub fn setup_panic() {
    setup_panic! {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    messages: {
            colors: (Color::Red, Color::Red, Color::Red),
            head: "%(name) %(version) crashed",
            body: "Report generated at %(file_path)",
            footer: "Waaa"
    }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    rules: Vec<Rule>,
}

impl Config {
    pub fn get_matching_rules(&self) -> Vec<ColorFn> {
        self.rules
            .iter()
            .map(|rule| rule.map_to_color_fn())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
struct Rule {
    regex: String,
    foreground_color: String,
    background_color: Option<String>,
}

pub type ColorFn<'a> = Box<dyn Fn(&String, &mut dyn Write) -> bool + 'a>;

impl Rule {
    fn map_to_color_fn(&self) -> ColorFn<'_> {
        Box::new(|line: &String, output: &mut dyn Write| {
            let regex = Regex::new(&self.regex).expect("Error parsing regex");

            if !regex.is_match(line.as_str()) {
                return false;
            }

            let foreground_color: colored::Color = self.foreground_color.clone().into();
            let mut line = line.color(foreground_color);
            if let Some(background_color) = &self.background_color {
                let background_color: colored::Color = background_color.clone().into();
                line = line.on_color(background_color);
            }
            write!(output, "{}", line).expect("Error writing to stdout");

            true
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rules: vec![
                Rule {
                    regex: "ERROR".into(),
                    foreground_color: "red".into(),
                    background_color: None,
                },
                Rule {
                    regex: "WARN".into(),
                    foreground_color: "magenta".into(),
                    background_color: None,
                },
                Rule {
                    regex: "INFO".into(),
                    foreground_color: "yellow".into(),
                    background_color: None,
                },
                Rule {
                    regex: "DEBUG".into(),
                    foreground_color: "green".into(),
                    background_color: None,
                },
            ],
        }
    }
}

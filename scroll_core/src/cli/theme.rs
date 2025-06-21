use ansi_term::{Colour, Style};
use clap::ValueEnum;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ThemeKind {
    Dark,
    Light,
}

#[derive(Clone)]
pub struct Theme {
    pub prompt_user: Style,
    pub prompt_agent: Style,
}

impl FromStr for ThemeKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dark" => Ok(ThemeKind::Dark),
            "light" => Ok(ThemeKind::Light),
            other => Err(format!("unknown theme '{other}'")),
        }
    }
}

pub fn theme_parse(s: &str) -> Result<ThemeKind, String> {
    s.parse()
}

impl ThemeKind {
    pub fn styles(self) -> Theme {
        match self {
            ThemeKind::Dark => Theme {
                prompt_user: Colour::Green.bold(),
                prompt_agent: Colour::Blue.bold(),
            },
            ThemeKind::Light => Theme {
                prompt_user: Colour::Purple.bold(),
                prompt_agent: Colour::Cyan.bold(),
            },
        }
    }
}

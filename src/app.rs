use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name="PassGen")]
#[command(version, about="Generate Diceware/XKCD-style passwords", long_about = None)]
pub struct App {
    /// Number of words in password. Minimum = 3
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(3..), default_value_t = 3)]
    pub number: u8,

    /// Generate non-interactively
    #[arg(long="no-reroll", action=ArgAction::SetFalse)]
    pub no_reroll: bool

}
//! Scripts for Helix Golf

use std::env;

use miette::miette;

mod command;
mod generate_tape_file;
mod parse_example;
use command::Command;
mod generate_helix_config;
mod parse_helix_keys;

fn main() -> miette::Result<()> {
    env::args()
        .nth(1)
        .ok_or(Command::ERROR)
        .and_then(|arg| arg.parse::<Command>())
        .map_err(|err| miette!("{err}"))?
        .execute()
}

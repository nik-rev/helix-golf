use std::env;

use action::Action;
use miette::miette;

mod action;
mod example;
mod helix_config;
mod helix_keys;

pub fn main() -> miette::Result<()> {
    env::args()
        .nth(1)
        .ok_or(Action::ERROR)
        .and_then(|arg| arg.parse::<Action>())
        .map_err(|err| miette!("{err}"))?
        .execute()
}

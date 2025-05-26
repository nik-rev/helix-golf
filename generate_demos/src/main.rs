use std::{fs, path::Path, process::Command};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use structure::Example;
use tap::Pipe as _;

mod helix_config;
mod helix_parse_keys;
mod key_event_display;
mod structure;

pub fn main() -> miette::Result<()> {
    let examples = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("src")
        .pipe(fs::read_dir)
        .unwrap()
        .flatten()
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && (entry.file_name().to_str().unwrap() != "SUMMARY.md")
        })
        .map(|entry| Example::try_new(entry.path(), entry.path().pipe(fs::read_to_string).unwrap()))
        .collect::<Result<Vec<Example>, _>>()?;

    let generated = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("src")
        .join("generated");
    helix_config::generate(&generated.join("helix-config.toml"));

    examples.par_iter().for_each(|example| {
        let stem = example.path.file_stem().unwrap().to_str().unwrap();
        let contents = key_event_display::generate_tape_file_from_helix_key_sequence(
            &example.command,
            stem,
            &example.ext,
        )
        .unwrap();

        let tape = generated.join(format!("{stem}.tape"));

        fs::write(&tape, contents).unwrap();

        fs::write(
            generated.join(format!("{stem}.{}", example.ext)),
            &example.before,
        )
        .unwrap();

        Command::new("vhs")
            .arg(tape)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    });

    Ok(())
}

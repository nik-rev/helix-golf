use std::{
    collections::HashSet,
    fs, iter,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

use miette::{Context, IntoDiagnostic, ensure, miette};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use structure::Example;
use tap::Pipe as _;

mod helix_config;
mod helix_parse_keys;
mod key_event_display;
mod structure;

static ROOT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("src"));
static GENERATED_DIR: LazyLock<PathBuf> = LazyLock::new(|| ROOT_DIR.join("generated"));

pub fn main() -> miette::Result<()> {
    // User can pass examples which will be ignored
    let examples_to_ignore: HashSet<_> = std::env::args().skip(1).collect();

    let examples = fs::read_dir(&*ROOT_DIR)
        .map_err(|err| miette!("failed to read {}: {err}", ROOT_DIR.display()))?
        .flatten()
        .filter(|entry| {
            entry.file_type().is_ok_and(|ft| ft.is_file())
                && (entry.path().file_stem().unwrap().to_str().unwrap() != "SUMMARY")
                && if examples_to_ignore.is_empty() {
                    true
                } else {
                    examples_to_ignore.contains(entry.path().file_stem().unwrap().to_str().unwrap())
                }
        })
        .map(|entry| Example::try_new(entry.path(), entry.path().pipe(fs::read_to_string).unwrap()))
        .collect::<Result<Vec<Example>, _>>()?;

    helix_config::generate(&GENERATED_DIR.join("helix-config.toml"));

    examples
        .par_iter()
        .try_for_each(|example| -> Result<(), miette::Error> {
            let stem = example
                .path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .with_context(|| format!("{} is missing stem", example.path.display()))?;

            println!("Rendering example `{stem}`...");
            let contents = key_event_display::generate_tape_file_from_helix_key_sequence(
                &example.command,
                stem,
                &example.ext,
            )?;

            let tape = GENERATED_DIR.join(format!("{stem}.tape"));

            fs::write(&tape, contents).map_err(|err| {
                miette!(
                    "Failed to create `{}` for example {stem}: {err}",
                    tape.display()
                )
            })?;

            fs::write(
                GENERATED_DIR.join(format!("{stem}.{}", example.ext)),
                &example.before,
            )
            .map_err(|err| {
                miette!(
                    "Failed to create `Before` for example {stem}.{}: {err}",
                    example.ext
                )
            })?;

            ensure!(
                which::which("vhs").is_ok(),
                "You need to install `vhs` in order to generate the demos"
            );

            Command::new("vhs")
                .arg(tape)
                .spawn()
                .into_diagnostic()?
                .wait()
                .into_diagnostic()?;

            println!("Finished rendering `{stem}`.");

            Ok(())
        })?;

    Ok(())
}

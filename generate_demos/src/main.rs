use std::{
    collections::HashSet,
    fs,
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
            let example_name = example
                .path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .with_context(|| format!("{} is missing stem", example.path.display()))?;

            println!("Rendering example `{example_name}`...");
            let contents = key_event_display::generate_tape_file_from_helix_key_sequence(
                &example.command,
                example_name,
                &example.ext,
            )?;

            let tape = GENERATED_DIR.join(format!("{example_name}.tape"));

            // Create .tape file
            //
            // These are the commands inputted into `vhs`
            fs::write(&tape, contents).map_err(|err| {
                miette!(
                    "Failed to create `{}` for example `{example_name}`: {err}",
                    tape.display()
                )
            })?;

            // First, this file has contents Before
            //
            // as we modify it, it'll have the contents that we expect from After
            fs::write(
                GENERATED_DIR.join(format!("{example_name}.{}", example.ext)),
                &example.before,
            )
            .map_err(|err| {
                miette!(
                    "Failed to create `Before` for example `{example_name}.{}`: {err}",
                    example.ext
                )
            })?;

            ensure!(
                which::which("vhs").is_ok(),
                "You need to install `vhs` in order to generate the demos"
            );

            // Generate the .mp4 file preview
            Command::new("vhs")
                .arg(tape)
                .spawn()
                .into_diagnostic()?
                .wait()
                .into_diagnostic()?;

            println!("Finished rendering `{example_name}`.");

            pretty_assertions::assert_str_eq!(
                fs::read_to_string(GENERATED_DIR.join(format!("{example_name}.{}", example.ext)))
                    .unwrap()
                    .trim(),
                example.after.trim(),
                "example {example_name}"
            );

            println!("Example {example_name} has been successfully tested.");

            Ok(())
        })?;

    println!("All examples have been successfully rendered and tested.");

    Ok(())
}

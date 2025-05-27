use std::fmt::Write as _;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

// use indicatif::{MultiProgress, ProgressStyle};
use miette::{IntoDiagnostic, ensure, miette};
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
    // let progress_bars = MultiProgress::new();
    // let styles = ProgressStyle::with_template(
    //     "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    // )
    // .unwrap()
    // .progress_chars("##-");

    // User can pass examples which will be ignored
    let examples_to_ignore: HashSet<_> = std::env::args().skip(1).collect();

    fs::remove_dir_all(&*GENERATED_DIR).unwrap();
    fs::create_dir_all(&*GENERATED_DIR).unwrap();

    let mut examples = fs::read_dir(&*ROOT_DIR)
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

    // We want to sort examples from smallest command count to largest
    examples.sort_unstable_by(|a, b| a.key_events.len().cmp(&b.key_events.len()));

    helix_config::generate();

    examples
        .iter()
        .fold(
            String::from(
                "<!-- @generated This file is generated. Do not edit it by hand. -->

# Summary\n\n",
            ),
            |mut summary_md, example| {
                writeln!(
                    &mut summary_md,
                    "- [{}]({})",
                    example.title,
                    example.path.file_name().unwrap().to_string_lossy()
                )
                .unwrap();
                summary_md
            },
        )
        .pipe(|summary_md| fs::write(ROOT_DIR.join("SUMMARY.md"), summary_md))
        .unwrap();

    examples
        .par_iter()
        .try_for_each(|example| -> Result<(), miette::Error> {
            let name = &example.name;
            let ext = &example.ext;

            println!("Rendering example `{name}`...");
            let contents = example.to_string();

            let tape = GENERATED_DIR.join(format!("{name}.tape"));

            // Create .tape file
            //
            // These are the commands inputted into `vhs`
            fs::write(&tape, contents).map_err(|err| {
                miette!(
                    "Failed to create `{}` for example `{name}`: {err}",
                    tape.display()
                )
            })?;

            let modification_file = GENERATED_DIR.join(format!("{name}.{ext}"));

            // First, this file has contents Before
            //
            // as we modify it, it'll have the contents that we expect from After
            fs::write(&modification_file, &example.before).map_err(|err| {
                miette!("Failed to create `Before` for example `{name}.{ext}`: {err}",)
            })?;

            ensure!(
                which::which("vhs").is_ok(),
                "You need to install `vhs` in order to generate the demos"
            );

            // Generate the .mp4 file preview
            let mut command = Command::new("vhs")
                .arg(tape)
                .stderr(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
                .into_diagnostic()?;

            let stdout = command.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                println!("{line:?}");
            }

            println!("Finished rendering `{name}`.");

            // Assert that the `## Before` code block is equal to the `## After` code block
            // once we have executed the commands in `## Commands` code block.
            pretty_assertions::assert_str_eq!(
                fs::read_to_string(modification_file)
                    .expect("read to not fail, because file exists as we have just written to it earlier")
                    .trim(),
                example.after.trim(),
                "example {name}"
            );

            println!("Example {name} has been successfully tested.");

            Ok(())
        })?;

    println!("All examples have been successfully rendered and tested.");

    Ok(())
}

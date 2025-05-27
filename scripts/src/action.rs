use std::env;
use std::fmt::Write as _;
use std::str::FromStr;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

use crate::example::Example;
use miette::{IntoDiagnostic, ensure, miette};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tap::Pipe as _;

/// Source directory for the mdbook content files
static ROOT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("src"));

/// Directory where we place all of the generated files
pub static GENERATED_DIR: LazyLock<PathBuf> = LazyLock::new(|| ROOT_DIR.join("generated"));

/// The action that the binary should execute
#[derive(Clone, Copy)]
pub enum Action {
    /// Parse all examples, to make sure they conform to the required structure
    Validate,
    /// 1. Perform `Validate`
    /// 2. Generate demo `.mp4` files
    /// 3. Test that each demo is correct
    GenerateDemos,
}

impl Action {
    pub const ERROR: &str = "Expected either `validate` or `generate-demos` as the first argument";

    pub fn execute(self) -> miette::Result<()> {
        match self {
            Action::Validate => validate().map(drop),
            Action::GenerateDemos => validate()?.pipe(generate_demos),
        }
    }
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validate" => Ok(Self::Validate),
            "generate-demos" => Ok(Self::GenerateDemos),
            _ => Err("Expected either `validate` or `generate-demos` as the first argument"),
        }
    }
}

pub fn generate_demos(examples: Vec<Example>) -> miette::Result<()> {
    // Use a custom helix config to ensure reproducibility
    //
    // This is also necessary because VHS cannot handle some
    crate::helix_config::generate();

    examples
        .par_iter()
        .try_for_each(|example| -> Result<(), miette::Error> {
            let name = &example.name;
            let ext = &example.ext;

            let tape_contents = example.to_string();

            let tape_file = GENERATED_DIR.join(format!("{name}.tape"));

            // Create .tape file
            //
            // These are the commands inputted into `vhs`
            fs::write(&tape_file, tape_contents).map_err(|err| {
                miette!(
                    "Failed to create `{}` for example `{name}`: {err}",
                    tape_file.display()
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
            Command::new("vhs")
                .arg(tape_file)
                .spawn()
                .into_diagnostic()?
                .wait()
                .into_diagnostic()?;

            // Assert that the `## Before` code block is equal to the `## After` code block
            // once we have executed the commands in `## Commands` code block.
            pretty_assertions::assert_str_eq!(
                fs::read_to_string(modification_file)
                    .expect(
                        "read to not fail, because file exists as \
                        we have just written to it earlier"
                    )
                    .trim(),
                example.after.trim(),
                "example `{name}`"
            );

            println!("Example `{name}` has been successfully tested.");

            Ok(())
        })?;

    println!("All examples have been successfully rendered and tested.");

    Ok(())
}

pub fn validate() -> miette::Result<Vec<Example>> {
    // If user passes any examples, those will be the only ones that are included.
    //
    // If no examples are passed, then include everything
    let only_include_these_examples: HashSet<_> = env::args()
        // 1. skip binary name
        // 2. skip argument type
        .skip(2)
        .collect();

    fs::remove_dir_all(&*GENERATED_DIR)
        .and_then(|_| fs::create_dir_all(&*GENERATED_DIR))
        .map_err(|err| miette!("failed cleaning the generated directory: {err}"))?;

    let mut examples = Example::parse_all(&ROOT_DIR, only_include_these_examples)?;

    // We want to sort examples from smallest command count to largest
    examples.sort_unstable_by(|a, b| a.key_events.len().cmp(&b.key_events.len()));

    examples
        .iter()
        .try_fold(
            String::from(
                "<!-- @generated This file is generated. Do not edit it by hand. -->

# Summary\n\n",
            ),
            |mut summary_md, example| -> miette::Result<String> {
                let name = &example.name;
                let title = &example.title;

                writeln!(&mut summary_md, "- [{title}]({name}.md)",).map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                Ok(summary_md)
            },
        )?
        .pipe(|summary_md| fs::write(ROOT_DIR.join("SUMMARY.md"), summary_md))
        .map_err(|err| miette!("Failed to write SUMMARY.md: {err}"))?;

    Ok(examples)
}

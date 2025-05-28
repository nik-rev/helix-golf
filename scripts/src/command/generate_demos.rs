//! Generate demo `.mp4` files for each example

use miette::{IntoDiagnostic as _, ensure, miette};
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::{fs, process::Command};

use crate::{command::GENERATED_DIR, parse_example::Example};

/// Generate `.mp4` files for each command
pub fn generate_demos(examples: &[Example]) -> miette::Result<()> {
    // Use a custom helix config to ensure reproducibility
    //
    // This is also necessary because VHS cannot handle some
    crate::generate_helix_config::generate();

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
                "ERROR (command `vhs` not found): You need to \
                install `vhs` in order to generate the demos"
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

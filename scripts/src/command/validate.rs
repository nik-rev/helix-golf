//! Validate that all of the examples adhere to a certain structure

use std::{collections::HashSet, env, fmt::Write as _, fs};

use crate::{
    command::{GENERATED_DIR, ROOT_DIR},
    parse_example::Example,
};
use miette::miette;
use tap::Pipe as _;

/// Make sure eacrh example has the required structure
pub fn validate() -> miette::Result<Vec<Example>> {
    // If user passes any examples, those will be the only ones that are included.
    //
    // If no examples are passed, then include everything
    let only_include_these_examples: HashSet<_> = env::args()
        // 1. skip binary name
        // 2. skip argument type
        .skip(2)
        .collect();

    fs::create_dir_all(&*GENERATED_DIR)
        .and_then(|_| fs::remove_dir_all(&*GENERATED_DIR))
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

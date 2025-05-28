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
    examples.sort_by(|a, b| a.key_events.len().cmp(&b.key_events.len()));

    examples
        .iter()
        .try_fold(
            (
                String::from(
                    "<!-- @generated This file is generated. Do not edit it by hand. -->

# Helix Golf

Helix Golf is a collection of refactoring examples using the [Helix Editor](https://github.com/helix-editor/helix), a next generation terminal IDE written in Rust.

Each example is described in-depth, is tested using the latest version of Helix and has a satisfying video demo. Examples aren't just made-up, all of them were created from real situations.

In many cases the Helix Golf examples are much easier to understand _and come up with on your own_ than similar Vim Golf examples, while often being shorter due to multiple cursors being a core editing primitive in Helix.

This makes Helix a perfect swiss army knife text-editor for developers and anyone who seeks to become faster at editing text. It's not just about becoming more productive - it's also really fun!

# Demo for each example\n\n",
                ),
                String::from(
                    "<!-- @generated This file is generated. Do not edit it by hand. -->

# Summary

- [Helix Golf - Introduction](introduction.md)\n",
                ),
            ),
            |(mut all_previews, mut summary_md), example| -> miette::Result<(String, String)> {
                let name = &example.name;
                let title = &example.title;

                writeln!(&mut summary_md, "- [{title}]({name}.md)",).map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                writeln!(
                    &mut all_previews,
                    "## [{title}]({name}.md)

{desc}

<video autoplay controls loop>
  <source src=\"generated/{name}.mp4\">
</video>\n\n",
                    desc = example.description.as_deref().unwrap_or("")
                )
                .map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                Ok((all_previews, summary_md))
            },
        )?
        .pipe(|(all_previews, summary_md)| {
            fs::write(ROOT_DIR.join("SUMMARY.md"), summary_md)
                .map_err(|err| miette!("Failed to write `SUMMARY.md`: {err}"))
                .map(|_| fs::write(ROOT_DIR.join("introduction.md"), all_previews))
        })?
        .map_err(|err| miette!("Failed to write `introduction.md`: {err}"))?;

    Ok(examples)
}

//! Preprocessor for mdbook that adds the example video to each page

use std::{env, io};

use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use miette::miette;
use tap::Pipe as _;

pub fn mdbook_preprocessor() -> miette::Result<()> {
    // 1. Skip the binary name
    // 2. Skip the first command `mdbook-preprocessor` which signals this binary to
    // run the markdown preprocessor
    match env::args().nth(2).as_deref() {
        Some("supports") => {
            // Supports all renderers
            return Ok(());
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    CmdPreprocessor::parse_input(io::stdin())
        .map_err(|err| miette!("failed to parse mdbook input: {err}"))?
        .pipe(|(ctx, book)| GolfPreprocessor.run(&ctx, book))
        .map_err(|err| miette!("failed to run the helix-golf mdbook preprocessor: {err}"))?
        .pipe(|book| serde_json::to_writer(io::stdout(), &book))
        .map_err(|err| miette!("failed to write the modified mdbook: {err}"))
}

struct GolfPreprocessor;

impl Preprocessor for GolfPreprocessor {
    fn name(&self) -> &str {
        "helix-golf-preprocessor"
    }

    fn run(
        &self,
        _ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        book.for_each_mut(|book_item| {
            if let mdbook::BookItem::Chapter(chapter) = book_item {
                if let (Some(name), Some(start)) = (
                    chapter
                        .path
                        .as_ref()
                        .and_then(|path| path.file_stem())
                        .and_then(|stem| stem.to_str()),
                    chapter.content.find("## Command"),
                ) {
                    let (before, after) = chapter.content.split_at(start);

                    chapter.content = format!(
                        r#"
{before}

## Preview

<video controls>
  <source src="generated/{name}.mp4" type="video/mp4">
</video>

{after}"#
                    );
                }
            }
        });

        Ok(book)
    }
}

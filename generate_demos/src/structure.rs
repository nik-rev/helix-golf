use std::path::PathBuf;

use markdown::{
    ParseOptions,
    mdast::{Code, Heading, List, Node, Text},
    unist::{Point, Position},
};
use miette::{NamedSource, SourceSpan};

/// The current element that we are expecting.
#[derive(Clone)]
enum Expecting {
    /// Title of the example.
    /// The snake_case conversion of it needs to be equal to the file name.
    ///
    /// If file is called `text_into_array.md`:
    ///
    /// ```md
    /// # Text into Array
    /// ```
    Title(Position),
    /// H2 with contents "Before"
    ///
    /// ```md
    /// ## Before
    /// ```
    TitleBefore(Position),
    /// The code block which represents
    /// the example BEFORE it was transformed
    ///
    /// ````md
    /// ```md
    /// Hello
    /// This
    /// Is
    /// Helix
    /// ```
    /// ````
    CodeBefore(Position),
    /// H2 with contents "After"
    ///
    /// ```md
    /// ## After
    /// ```
    TitleAfter(Position),
    /// Code block which represents the example
    /// AFTER it was transformed
    ///
    /// ````md
    /// ```js
    /// ["Hello", "This", "Is", "Helix"];
    /// ```
    /// ````
    CodeAfter(Position),
    /// H2 with contents "Preview"
    ///
    /// Contains the actual video inside.
    ///
    /// ```md
    /// ## Preview
    /// ```
    TitlePreview(Position),
    /// H2 with contents "Command"
    ///
    /// ```md
    /// ## Command
    /// ```
    TitleCommand(Position),
    /// Code block which contains the entire
    /// key sequence.
    ///
    /// ````md
    /// ```
    /// %<A-s>ms"<A-J>i,<esc>xms ms[
    /// ```
    /// `````
    CodeCommand(Position),
    /// Numbered list which describes each step
    ///
    /// ```md
    /// 1. `%` selects full file
    /// 1. `<A-s>` split selection into multiple selections on newlines
    /// 1. `ms"` surrounds each word with `"`'s
    /// 1. `<A-J>i,` join lines inside selection, select the inserted space, and insert `,`'s
    /// 1. `<esc>xms ` enter normal mode, select line and surround by spaces
    /// 1. `ms[` surround by `[]`
    /// ```
    ListCommand(Position),
    /// The required structure was met.
    Finished,
}

impl Expecting {
    pub fn with_pos(self, pos: Position) -> Self {
        match self {
            Self::Title(_) => Self::Title(pos),
            Self::TitleBefore(_) => Self::TitleBefore(pos),
            Self::CodeBefore(_) => Self::CodeBefore(pos),
            Self::TitleAfter(_) => Self::TitleAfter(pos),
            Self::CodeAfter(_) => Self::CodeAfter(pos),
            Self::TitlePreview(_) => Self::TitlePreview(pos),
            Self::TitleCommand(_) => Self::TitleCommand(pos),
            Self::CodeCommand(_) => Self::CodeCommand(pos),
            Self::ListCommand(_) => Self::ListCommand(pos),
            Self::Finished => Self::Finished,
        }
    }

    pub fn expected(self) -> Option<(Position, &'static str)> {
        Some(match self {
            Self::Title(pos) => (pos, "expected heading: `# ...`"),
            Self::TitleBefore(pos) => (pos, "expected heading `## Before`"),
            Self::CodeBefore(pos) => (pos, "expected code block after `## Before`"),
            Self::TitleAfter(pos) => (pos, "expected heading `## After`"),
            Self::CodeAfter(pos) => (pos, "expected code block after `## After`"),
            Self::TitlePreview(pos) => (pos, "expected heading `## Preview`"),
            Self::TitleCommand(pos) => (pos, "expected heading `## Title`"),
            Self::CodeCommand(pos) => (pos, "expected code block after `## Title`"),
            Self::ListCommand(pos) => (pos, "expected numbered list describing each command"),
            Self::Finished => return None,
        })
    }

    pub fn next(&mut self, pos: Position) {
        *self = match self {
            Self::Title(..) => Self::TitleBefore(pos),
            Self::TitleBefore(..) => Self::CodeBefore(pos),
            Self::CodeBefore(..) => Self::TitleAfter(pos),
            Self::TitleAfter(..) => Self::CodeAfter(pos),
            Self::CodeAfter(..) => Self::TitlePreview(pos),
            Self::TitlePreview(..) => Self::TitleCommand(pos),
            Self::TitleCommand(..) => Self::CodeCommand(pos),
            Self::CodeCommand(..) => Self::ListCommand(pos),
            Self::ListCommand(..) => Self::Finished,
            Self::Finished => unreachable!(),
        };
    }
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("Invalid structure of example, please see README.md for correct structure")]
struct InvalidStructure {
    #[source_code]
    src: NamedSource<String>,
    why: String,
    #[label("{why}")]
    span: SourceSpan,
}

/// Represents a single Helix Golf example
#[derive(Default, Debug)]
pub struct Example {
    /// Location of the file
    pub path: PathBuf,
    /// Contents of the file before the `command`
    pub before: String,
    /// Contents of the file after the `command`
    pub after: String,
    /// Command to go from `before` -> `after`
    pub command: String,
    /// Extension of the file
    pub ext: String,
}

impl Example {
    pub fn try_new(path: PathBuf, markdown: String) -> miette::Result<Self> {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        markdown::to_mdast(&markdown, &ParseOptions::default())
            .unwrap()
            .children()
            .unwrap()
            .iter()
            .try_fold(
                (
                    Expecting::Title(Position {
                        start: Point {
                            line: 0,
                            column: 0,
                            offset: 0,
                        },
                        end: Point {
                            line: 0,
                            column: 0,
                            offset: 0,
                        },
                    }),
                    Example {
                        path: path.clone(),
                        ..Default::default()
                    },
                ),
                |(mut expecting, mut example), child| {
                    let err = |pos: &Option<Position>| {
                        // NOTE: These `clone`s are cheap.
                        // `Position` could be `Copy` as it is just 6 `usize`,
                        // but unfortunately `miette` does not `#[derive(Copy)] Position`
                        expecting
                            .clone()
                            .with_pos(pos.clone().unwrap())
                            .expected()
                            .unwrap()
                    };
                    match expecting {
                        Expecting::Title(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 1,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { position, .. })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::TitleBefore(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 2,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { value, position })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                if value != "Before" {
                                    return Err(err(position));
                                }

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::CodeBefore(_) => {
                            if let Node::Code(Code {
                                value, position, ..
                            }) = child
                            {
                                example.before = value.to_string();

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::TitleAfter(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 2,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { value, position })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                if value != "After" {
                                    return Err(err(position));
                                }

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::CodeAfter(_) => {
                            if let Node::Code(Code {
                                value,
                                position,
                                lang,
                                ..
                            }) = child
                            {
                                example.after = value.to_string();
                                example.ext = lang.clone().unwrap_or_default();

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::TitlePreview(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 2,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { value, position })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                if value != "Preview" {
                                    return Err(err(position));
                                }

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::TitleCommand(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 2,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { value, position })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                if value != "Command" {
                                    return Err(err(position));
                                }

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::CodeCommand(_) => {
                            if let Node::Code(Code {
                                value, position, ..
                            }) = child
                            {
                                example.command = value.replace("\n", "");

                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::ListCommand(_) => {
                            if let Node::List(List { position, .. }) = child {
                                expecting.next(position.clone().unwrap());
                            }
                        }
                        Expecting::Finished => (),
                    };
                    Ok((expecting, example))
                },
            )
            .and_then(|(expecting, example)| {
                if let Some((pos, why)) = expecting.expected() {
                    Err((pos, why))
                } else {
                    Ok(Ok(example))
                }
            })
            .map_err(|(Position { start, end }, info)| {
                let length = end.offset - start.offset;
                InvalidStructure {
                    src: NamedSource::new(file_name, markdown),
                    span: (start.offset, length).into(),
                    why: info.to_string(),
                }
            })?
    }
}

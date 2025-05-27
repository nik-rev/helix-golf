//! Ensure that each markdown file corresponds to the expected structure

use std::{fmt::Display, path::PathBuf};

use markdown::{
    ParseOptions,
    mdast::{Code, Heading, InlineCode, List, Node, Text},
    unist::{Point, Position},
};
use miette::{Context, NamedSource, SourceSpan};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::helix_parse_keys::{self, KeyEvent};

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
            Self::Title(_) => Self::TitleBefore(pos),
            Self::TitleBefore(_) => Self::CodeBefore(pos),
            Self::CodeBefore(_) => Self::TitleAfter(pos),
            Self::TitleAfter(_) => Self::CodeAfter(pos),
            Self::CodeAfter(_) => Self::TitlePreview(pos),
            Self::TitlePreview(_) => Self::TitleCommand(pos),
            Self::TitleCommand(_) => Self::CodeCommand(pos),
            Self::CodeCommand(_) => Self::ListCommand(pos),
            Self::ListCommand(_) => Self::Finished,
            Self::Finished => unreachable!(),
        };
    }
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("Invalid structure of example, please see README.md for correct structure")]
struct InvalidStructure {
    #[source_code]
    src: NamedSource<String>,
    reason: String,
    #[label("{reason}")]
    span: SourceSpan,
}

/// Represents a single Helix Golf example
#[derive(Default, Debug)]
pub struct Example {
    /// Location of the file
    pub path: PathBuf,
    /// Level 1 heading
    pub title: String,
    /// Contents of the file before the `command`
    pub before: String,
    /// Contents of the file after the `command`
    pub after: String,
    /// Command to go from `before` -> `after`
    pub command: String,
    /// Parsed `command` into a structure that can be converted into a `.tape` file
    pub key_events: Vec<KeyEvent>,
    /// Name of the example. This is name of the file, excluding the `.md` extension.
    pub name: String,
    /// Extension of the file
    pub ext: String,
}

impl Display for Example {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"Output src/generated/{name}.mp4
Require hx

Hide
Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 600
Set Padding 0
Set Theme "Catppuccin Mocha"
Set TypingSpeed 400ms
Type "hx -c src/generated/helix-config.toml src/generated/{name}.{ext}"
Enter
Show
"#,
            name = self.name,
            ext = self.ext
        )?;

        for key in &self.key_events {
            writeln!(f, "{key}")?;
        }

        f.write_str(
            r#"
Escape
Type ","

Hide
Type ":w!"
Enter
Show

Sleep 2s"#,
        )
    }
}

impl Example {
    pub fn try_new(path: PathBuf, markdown: String) -> miette::Result<Self> {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("filename cannot end with `..`")?;
        let file_stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .context("missing filename")?;

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
                            .map(|(pos, s)| (pos, s.to_string()))
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
                                let Some(Node::Text(Text { position, value })) = children.first()
                                else {
                                    return Err(err(position));
                                };

                                example.title = value.to_string();

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
                                #[allow(clippy::nonminimal_bool, reason = "more readable")]
                                if value
                                    .chars()
                                    .collect::<Vec<_>>()
                                    .par_windows(3)
                                    .any(|windows| {
                                        if let [a, b, c] = windows {
                                            // 1 newline is not allowed
                                            (*a != '\n' && *b == '\n' && *c != '\n')
                                            // 3 newlines is not allowed
                                            || (*a == '\n' && *b == '\n' && *c == '\n')
                                        } else {
                                            unreachable!()
                                        }
                                    })
                                {
                                    return Err((
                                        position.clone().unwrap(),
                                        "For each line break, use exactly 2 newlines".to_string(),
                                    ));
                                }

                                let position = position.clone().unwrap();

                                if let Some(line) = value.lines().find(|line| line.len() > 30) {
                                    return Err((
                                        position,
                                        format!(
                                            "Each line in code block \
                                         after `## Command` \
                                         should be at most 30 \
                                         characters long.\nThis helps with readability \
                                         on smaller devices.\n\n\
                                         This line is more than 30 \
                                         characters long (is {} chars long):\n  \
                                           {line}\n\nBreak it with two newlines.",
                                            line.len()
                                        )
                                        .to_string(),
                                    ));
                                }

                                example.command = value.replace("\n", "");

                                expecting.next(position);
                            }
                        }
                        Expecting::ListCommand(_) => {
                            if let Node::List(List {
                                position,
                                ordered: true,
                                children,
                                ..
                            }) = child
                            {
                                let mut concatenated_inline_code = String::new();
                                for child in children {
                                    if let Node::Code(Code { value, .. }) = child {
                                        concatenated_inline_code.push_str(value.trim());
                                    } else if let Some(children) = child.children() {
                                        for child in children {
                                            // each child in the List Item
                                            if let Node::Code(Code { value, .. }) = child {
                                                concatenated_inline_code.push_str(value.trim());
                                            } else {
                                                let inline_code_concatenated = child
                                                    .children()
                                                    .into_iter()
                                                    .flat_map(|children| children.iter())
                                                    // each child in the Paragraph
                                                    .flat_map(|child| {
                                                        // only care about `InlineCode`
                                                        if let Node::InlineCode(InlineCode {
                                                            value,
                                                            ..
                                                        }) = child
                                                        {
                                                            Some(value)
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .fold(String::new(), |total, inline_code| {
                                                        total + inline_code
                                                    });
                                                concatenated_inline_code
                                                    .push_str(&inline_code_concatenated);
                                            }
                                        }
                                    }
                                }

                                if concatenated_inline_code != example.command {
                                    return Err((
                                        position.clone().unwrap(),
                                        format!(
                                            "Code blocks in the explanation list \
                                            must concatenate to the command.\n\n\
                                        if you concatenate all code blocks in \
                                        the list, you will get:\n  \
                                          {concatenated_inline_code}\n\n\
                                        but we expected to see contents of the \
                                        code block after `## Command`:\n  \
                                          {}",
                                            example.command
                                        ),
                                    ));
                                }

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
                    Err((pos, why.to_string()))
                } else {
                    Ok(Ok(example))
                }
            })
            .map_err(|(Position { start, end }, info)| {
                let length = end.offset - start.offset;
                InvalidStructure {
                    src: NamedSource::new(file_name, markdown),
                    span: (start.offset, length).into(),
                    reason: info.to_string(),
                }
            })?
            .and_then(|mut example| {
                example.key_events = helix_parse_keys::parse_keys(&example.command, file_stem)?;
                example.name = file_stem.to_string();
                Ok(example)
            })
    }
}

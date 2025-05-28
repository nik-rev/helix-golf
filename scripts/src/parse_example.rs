//! Ensure that each markdown file corresponds to the expected structure

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use markdown::{
    ParseOptions,
    mdast::{Code, Emphasis, Heading, InlineCode, Link, List, Node, Paragraph, Strong, Text},
    unist::{Point, Position},
};
use miette::{Context, NamedSource, SourceSpan, miette};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::parse_helix_keys::KeyEvent;

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
    /// Change the position of this node
    pub fn with_pos(self, pos: Position) -> Self {
        match self {
            Self::Title(_) => Self::Title(pos),
            Self::TitleBefore(_) => Self::TitleBefore(pos),
            Self::CodeBefore(_) => Self::CodeBefore(pos),
            Self::TitleAfter(_) => Self::TitleAfter(pos),
            Self::CodeAfter(_) => Self::CodeAfter(pos),
            Self::TitleCommand(_) => Self::TitleCommand(pos),
            Self::CodeCommand(_) => Self::CodeCommand(pos),
            Self::ListCommand(_) => Self::ListCommand(pos),
            Self::Finished => Self::Finished,
        }
    }

    /// Check that we have found all expected nodes.
    ///
    /// # Returns
    ///
    /// - `Some` if we are still expecting something.
    /// - `None` if `self == Self::Finished`
    pub fn check(self) -> Option<(Position, &'static str)> {
        Some(match self {
            Self::Title(pos) => (pos, "expected heading: `# ...`"),
            Self::TitleBefore(pos) => (pos, "expected heading `## Before`"),
            Self::CodeBefore(pos) => (pos, "expected code block after `## Before`"),
            Self::TitleAfter(pos) => (pos, "expected heading `## After`"),
            Self::CodeAfter(pos) => (pos, "expected code block after `## After`"),
            Self::TitleCommand(pos) => (pos, "expected heading `## Title`"),
            Self::CodeCommand(pos) => (pos, "expected code block after `## Title`"),
            Self::ListCommand(pos) => (pos, "expected numbered list describing each command"),
            Self::Finished => return None,
        })
    }

    /// The requirement for the current node that we are expecting has been met.
    ///
    /// Expect the next node.
    pub fn next(&mut self, pos: Position) {
        *self = match self {
            Self::Title(_) => Self::TitleBefore(pos),
            Self::TitleBefore(_) => Self::CodeBefore(pos),
            Self::CodeBefore(_) => Self::TitleAfter(pos),
            Self::TitleAfter(_) => Self::CodeAfter(pos),
            Self::CodeAfter(_) => Self::TitleCommand(pos),
            Self::TitleCommand(_) => Self::CodeCommand(pos),
            Self::CodeCommand(_) => Self::ListCommand(pos),
            Self::ListCommand(_) => Self::Finished,
            Self::Finished => Self::Finished,
        };
    }
}

/// The markdown file does not conform to the structure that we are expecting.
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("Invalid structure of example, please see README.md for correct structure")]
struct InvalidStructure {
    /// Contents of the markdown file
    #[source_code]
    src: NamedSource<String>,
    /// Why the error has occured
    reason: String,
    /// Region in the markdown file that the error points out
    #[label("{reason}")]
    span: SourceSpan,
}

/// Represents a single Helix Golf example
#[derive(Default, Debug)]
pub struct Example {
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
    /// Description of the example
    pub description: Option<String>,
}

impl Example {
    /// Reads multiple examples from the `root` directory.
    ///
    /// If `filter` is empty, parse all of the examples from `root`.
    /// Otherwise, parses all of the examples in `filter` that are available in the `root`.
    pub fn parse_all(root: &Path, filter: HashSet<String>) -> miette::Result<Vec<Self>> {
        fs::read_dir(root)
            .map_err(|err| miette!("failed to read {root}: {err}", root = root.display()))?
            .flatten()
            .filter(|entry| {
                let stem = entry.path();
                let stem = stem.file_stem().and_then(|stem| stem.to_str()).unwrap();

                entry.file_type().is_ok_and(|ft| ft.is_file())
                    // fully ignore these files, as we auto-generate them in a special way
                    && stem != "SUMMARY"
                    && stem != "introduction"
                    && if filter.is_empty() {
                        // include everything if no filter specified
                        true
                    } else {
                        // include only the stuff specified in the filter
                        filter.contains(stem)
                    }
            })
            .map(|entry| Example::parse(entry.path()))
            .collect()
    }

    /// Try to parse path of the given markdown file
    pub fn parse(path: PathBuf) -> miette::Result<Self> {
        let markdown = fs::read_to_string(&path)
            .map_err(|err| miette!("failed to read path {}: {err}", path.display()))?;

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
                    Example::default(),
                ),
                |(mut expecting, mut example), child| {
                    let expected_err_with_pos = |pos: &Option<Position>| {
                        // NOTE: These `clone`s are cheap.
                        // `Position` could be `Copy` as it is just 6 `usize`,
                        // but unfortunately `miette` does not `#[derive(Copy)] Position`
                        expecting
                            .clone()
                            .with_pos(pos.clone().unwrap())
                            .check()
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
                                    return Err(expected_err_with_pos(position));
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
                                    return Err(expected_err_with_pos(position));
                                };

                                if value != "Before" {
                                    return Err(expected_err_with_pos(position));
                                }

                                expecting.next(position.clone().unwrap());
                            // optional description
                            // After the `# Title` of the example
                            // but before the `## Before`
                            } else if let Node::Paragraph(Paragraph { children, .. }) = child {
                                fn inline_mdast_into_md_string(children: &[Node]) -> String {
                                    children.iter().fold(String::new(), |md, child| {
                                        let new = match child {
                                            Node::Emphasis(Emphasis { children, .. }) => {
                                                format!(
                                                    "_{}_",
                                                    inline_mdast_into_md_string(children)
                                                )
                                            }
                                            Node::Link(Link { children, url, .. }) => format!(
                                                "[{}]({url})",
                                                inline_mdast_into_md_string(children)
                                            ),
                                            Node::Text(Text { value, .. }) => value.to_string(),
                                            Node::InlineCode(InlineCode { value, .. }) => {
                                                format!("`{value}`")
                                            }
                                            Node::Strong(Strong { children, .. }) => format!(
                                                "**{}**",
                                                inline_mdast_into_md_string(children)
                                            ),
                                            // no modifications
                                            _ => return md,
                                        };
                                        format!("{md}{new}")
                                    })
                                }

                                example.description = Some(inline_mdast_into_md_string(children));
                            }
                        }
                        Expecting::CodeBefore(_) => {
                            if let Node::Code(Code {
                                value, position, ..
                            }) = child
                            {
                                example.before = if value.ends_with('\n') {
                                    value.to_string()
                                } else {
                                    format!("{value}\n")
                                };

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
                                    return Err(expected_err_with_pos(position));
                                };

                                if value != "After" {
                                    return Err(expected_err_with_pos(position));
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
                        Expecting::TitleCommand(_) => {
                            if let Node::Heading(Heading {
                                children,
                                depth: 2,
                                position,
                            }) = child
                            {
                                let Some(Node::Text(Text { value, position })) = children.first()
                                else {
                                    return Err(expected_err_with_pos(position));
                                };

                                if value != "Command" {
                                    return Err(expected_err_with_pos(position));
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

                                const MAX_LINE_LEN: usize = 60;

                                if let Some(line) =
                                    value.lines().find(|line| line.len() > MAX_LINE_LEN)
                                {
                                    return Err((
                                        position,
                                        format!(
                                            "Each line in code block \
                                         after `## Command` \
                                         should be at most {MAX_LINE_LEN} \
                                         characters long.\nThis helps with readability \
                                         on smaller devices.\n\n\
                                         This line is more than {MAX_LINE_LEN} \
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
                if let Some((pos, why)) = expecting.check() {
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
                example.key_events =
                    crate::parse_helix_keys::parse_keys(&example.command, file_stem)?;
                example.name = file_stem.to_string();
                Ok(example)
            })
    }
}

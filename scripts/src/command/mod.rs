//! The `Command` represents possible things this binary can do

mod generate_demos;
mod mdbook_preprocessor;
mod validate;

use std::str::FromStr;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use tap::Pipe as _;

/// Source directory for the mdbook content files
pub static ROOT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("src"));

/// Directory where we place all of the generated files
pub static GENERATED_DIR: LazyLock<PathBuf> = LazyLock::new(|| ROOT_DIR.join("generated"));

/// The action that the binary should execute
#[derive(Clone, Copy)]
pub enum Command {
    /// Parse all examples, to make sure they conform to the required structure
    Validate,
    /// 1. Perform `Validate`
    /// 2. Generate demo `.mp4` files
    /// 3. Test that each demo is correct
    GenerateDemos,
    /// Transforms each markdown file
    MdBookPreprocessor,
}

impl Command {
    pub const ERROR: &str = "Expected either `validate`, `generate-demos` or `mdbook-preprocessor` as the first argument";

    pub fn execute(self) -> miette::Result<()> {
        match self {
            Self::Validate => validate::validate().map(drop),
            Self::GenerateDemos => validate::validate()?.pipe_deref(generate_demos::generate_demos),
            Self::MdBookPreprocessor => mdbook_preprocessor::mdbook_preprocessor(),
        }
    }
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validate" => Ok(Self::Validate),
            "generate-demos" => Ok(Self::GenerateDemos),
            "mdbook-preprocessor" => Ok(Self::MdBookPreprocessor),
            _ => Err("Expected either `validate` or `generate-demos` as the first argument"),
        }
    }
}

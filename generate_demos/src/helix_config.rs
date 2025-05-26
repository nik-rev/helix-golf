//! The helix config used by the recordings.

use std::{fs, path::Path};

use tap::Pipe as _;

pub fn remap(mods: &str, ch: char) -> Option<String> {
    match (mods, ch) {
        // Remap alt keys because `vhs` cannot handle them
        ("Alt+", 's') => r#"Ctrl+"s""#.to_string(),
        ("Alt+", 'J') => r#"Ctrl+"m""#.to_string(),
        ("Alt+", '(') => r#"Ctrl+"y""#.to_string(),
        ("Alt+", ')') => r#"Ctrl+"g""#.to_string(),
        ("Alt+", ',') => r#"Ctrl+Shift+"m""#.to_string(),
        _ => return None,
    }
    .pipe(Some)
}

pub fn generate(path: &Path) {
    let remapped = r##"
# Original: Alt + s
C-s = "split_selection_on_newline"

# Original: Alt + J
C-m = "join_selections_space"

# Original: Alt + (
"C-y" = "rotate_selection_contents_backward"

# Original: Alt + )
"C-g" = "rotate_selection_contents_forward"

"C-M" = "remove_primary_selection"

"##;

    fs::write(
        path,
        format!(
            r#"theme = "catppuccin_mocha"
        
[keys.normal]
{remapped}

[keys.select]
{remapped}"#
        ),
    )
    .unwrap();
}

//! The helix config used by the recordings.

use std::fs;

use tap::Pipe as _;

pub fn remap(mods: &str, ch: char) -> Option<String> {
    match (mods, ch) {
        // Remap alt keys because `vhs` cannot handle them
        ("Alt+", 's') => r#"Ctrl+"s""#.to_string(),
        ("Alt+", 'J') => r#"Ctrl+"y""#.to_string(),
        ("Alt+", '(') => r#"Ctrl+"z""#.to_string(),
        _ => return None,
    }
    .pipe(Some)
}

pub fn generate() {
    let remapped = r##"
# Original: Alt + s
C-s = "split_selection_on_newline"

# Original: Alt + J
C-y = "join_selections_space"

# Original: Alt + (
C-z = "rotate_selection_contents_backward"

# Original: Alt + )
#"C-g" = "rotate_selection_contents_forward"

#"C-M" = "remove_primary_selection"

"##;

    fs::write(
        crate::action::GENERATED_DIR.join("helix-config.toml"),
        format!(
            r#"theme = "base16_transparent"
            
[editor]
true-color = false
auto-format = false
lsp.enable = false
            
[editor.gutters]
layout = ["line-numbers", "spacer"]

[keys.normal]
{remapped}

[keys.select]
{remapped}"#
        ),
    )
    .unwrap();
}

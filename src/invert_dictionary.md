# Invert Dictionary

Switch the key-value pairs of a dictionary.

## Before

```gdscript
var color_to_points = {
    "red" = 0,
    "orange" = 5,
    "yellow" = 10,
    "green" = 15,
    "blue" = 20,
    "purple" = 30,
    "black" = 50,
}
```

## After

```gdscript
var points_to_color = {
    0 = "red",
    5 = "orange",
    10 = "yellow",
    15 = "green",
    20 = "blue",
    30 = "purple",
    50 = "black",
}
```

## Command

```
webS_to_<enter><alt-(>

QjxhdT"PxS=<enter>_<alt-(>Q6q
```

1. `web` selects the second word without whitespace
1. `S` splits the selection on regex match. We type in `_to_` for the regex and hit `<enter>` to split it into two sections
1. `<alt-(>` rotates the contents of the selections
1. Pressing `Q` for the first time begins recording a macro
1. `j` moves down a line
1. `xh` selects the last comma on the current line
1. `d` deletes the selected comma and copies it
1. `T"` selects until (but not including) the closest previous double quote
1. `P` pastes the copied comma before the current selection
1. `xS` splits the whole line on regex match. We type in `=` for the regex and hit `<enter>` to split it into two sections
1. `_` trims trailing whitespace on all selections
1. `<alt-(>` rotates the contents of the selections
1. Pressing `Q` for the second time ends the recording of the macro
1. `6q` repeats the recorded macro sequence 6 times

# Invert Dictionary 2

Another way to switch the key-value pairs of a dictionary.

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

xt}S,|=<enter>_2<alt-(>
```

1. `web` selects the second word without whitespace
1. `S` splits the selection on regex match. We type in `_to_` for the regex and hit `<enter>` to split it into two sections
1. `<alt-(>` rotates the contents of the selections
1. `x` selects the whole line
1. `t}` selects until (but not including) the next "}" character
1. `S` splits the selection on regex match. We type in `,|=` for the regex and hit `<enter>` to split it into sub-selections
1. `_` trims trailing whitespace on all selections
1. `2<alt-(>` rotates the contents of selections, but only between pairs of selections.

# Function into Class

Convert 3 functions into a class with 3 methods.

## Before

```py
def calculate_area(length, width):
    result = length * width
    return result

def calculate_perimiter(length, width):
    result = 2 * (length + width)
    return result

def calculate_volume(length, width, height):
    result = length * width * height
    return result
```

## After

```py
class Calculator:
    @staticmethod
    def get_area(len, wid):
        return len * wid

    @staticmethod
    def get_perimiter(len, wid):
        return 2 * (len + wid)

    @staticmethod
    def get_volume(len, wid, hei):
        return len * wid * hei
```

## Command

```
%scalculate<enter>cget<esc>

O@staticmethod<esc>jj

vglyx<alt-d>xbRkxx

slength|width|height<enter>

bllled%>O<backspace>

class Calculator:
```

1.  `%` selects the entire file
1.  `s` searches inside the current selection and creates sub-selections based on a pattern. Input `calculate` then hit `<enter>` to make a selection on all instances of the word
1.  `c` then type `get` to change each "calculate" word into a "get"
1.  `<esc>` to go back to normal mode
1.  Use `O` to create an empty line above each cursor, write:

    ```
    @staticmethod
    ```

1.  Hit `<esc>` to go into normal mode.
1. `jj` moves each cursor down two lines
1. `vgl` selects the rest of each line past each cursor
1. `y` copies each selection
1. `x<alt-d>` selects each cursor's line and deletes the line without copying the selection
1. `xb` selects the last word of each cursor's line
1. `R` replaces each selection with the copied selections from earlier
1. `kxx` moves each cursor up one line and selects that line as well as the line below
1.  `s` brings up a prompt to select sub-selections by a given regex. Typing `length|width|height` followed by `<enter>` will select each instance of those 3 words: length, width, and height
1.  Our cursor is currently at the end of each word. Let's go to the beginning with `b`
1.  We want to keep the first 3 characters and discard the rest from each of the parameters. To do this, move to the 4th character with `lll`
1.  Use `e` to select until the end of each word and then `d` to delete it
1.  Select whole file with `%` and indent with `>`
1.  `O` creates a newline above and enters Insert mode, then `<backspace>` to delete an extra tab
1.  Write this:

    ```
    class Calculator:
    ```

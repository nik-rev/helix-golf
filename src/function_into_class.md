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

O@staticmethod

<esc>jxxs\w+<enter>s

length|width|height<enter>

bllled%sresult =<enter>C

<alt-(>;ddss<enter>

xd%>O<backspace>

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
1.  We need to select 2 lines below the current line, first go down with `j` and then press `xx` which will select the current line, and then select the next line
    In total we now have 3 cursors each with 2 lines selected, which includes the first line of the bodies of each function and the function definition themselves

1.  `s` brings up a prompt to select sub-selections by a given regex. The `\w+` regex selects each word, type it and then `<enter>`
1.  `s` again then type `length|width|height` followed by `<enter>`. This will look at the contents of the current selections, and create sub-selections where it finds the regex which means "length or width or height". So we select each instance of those 3 words
1.  Our cursor is currently at the end of each word. Let's go to the beginning with `b`
1.  We want to keep the first 3 characters and discard the rest from each of the parameters. To do this, move to the 4th character with `lll`
1.  Use `e` to select until the end of each word and then `d` to delete it
1.  Select the entire file again with `%` followed by `s` to bring up selection prompt again
1.  Write `result =` followed by `<enter>` to select all instances of that string
1.  `C` creates a new selection on the line directly below, for each cursor
1.  Use `<alt-(>` to rotate the _contents_ of the selection backward
1.  `;` collapses each cursor into a single selection
1.  `dd` deletes two characters on each of the 6 lines
1.  `s` to bring up the prompt, then input `s` followed by `<enter>` to select all "s" characters
1.  Select each of the lines with `x` followed by `d` to delete
1.  Select whole file with `%` and indent with `>`
1.  `O` creates a newline above and enters Insert mode, then `<backspace>` to delete an extra tab
1.  Write this:

    ```
    class Calculator:
    ```

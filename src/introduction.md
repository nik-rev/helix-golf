<!-- @generated This file is generated. Do not edit it by hand. -->

# Helix Golf

Helix Golf is a collection of refactoring examples using the
[Helix Editor](https://github.com/helix-editor/helix),
a next generation terminal IDE written in Rust.

Each example is described in-depth, is tested using the latest
version of Helix and has a satisfying video demo. Examples aren't
just made-up, all of them were created from real situations.

In many cases the Helix Golf examples are much easier to understand
_and come up with on your own_ than similar Vim Golf examples,
while often being shorter due to multiple cursors being a
core editing primitive in Helix.

This makes Helix a perfect swiss army knife text-editor
for developers and anyone who seeks to become faster at editing text.
It's not just about becoming more productive - it's also really fun!

# Demo for each example

The entire website and all examples are available in a single
code block you can copy-paste and work on locally:

<details>

<summary>All Examples (single markdown file)</summary>

````````````md
# snake_case to camelCase

Rename all fields to be camelCase.

## Before

```js
const user_profile = {
  first_name: "John",
  last_name: "Doe",
  birth_date: "1990-05-15",
  email_address: "john_doe@example.com",
  phone_number: "555-123-4567",
  mailing_address: {
    street_name: "Main Street",
    house_number: 123,
    apartment_unit: "4B",
    zip_code: "10001",
    city_name: "New York",
  },
};
```

## After

```js
const userProfile = {
  firstName: "John",
  lastName: "Doe",
  birthDate: "1990-05-15",
  emailAddress: "john_doe@example.com",
  phoneNumber: "555-123-4567",
  mailingAddress: {
    streetName: "Main Street",
    houseNumber: 123,
    apartmentUnit: "4B",
    zipCode: "10001",
    cityName: "New York",
  },
};
```

## Command

```
%s_<enter>5)<alt-,>d~
```

1. `%` selects the entire file
2. `s_<enter>` selects all underscores
3. `5)` rotates the main selection forward 5 times
4. `<alt-,>` removes the primary selection - the lone underscore we want to keep
5. `d` deletes the selections
6. `~` toggles the case

# Text into Array

Join newline-separated data into an array of strings

## Before

```
Hello
This
Is
Helix
```

## After

```js
["Hello", "This", "Is", "Helix"]
```

## Command

```
%<alt-s>ms"<alt-J>i,<esc>xms[
```

1. `%` selects full file
1. `<alt-s>` split selection into multiple selections on newlines
1. `ms"` surrounds each word with quotes
1. `<alt-J>i,` join lines inside selection, select the inserted space, and insert ","s
1. `<esc>xms[` surround by "[]"

# Export from Rust Module

Each module contains a function, which we want to export.

## Before

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;
```

## After

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;

pub use generate_demos::generate_demos;
pub use mdbook_preprocessor::mdbook_preprocessor;
pub use validate::validate;
```

## Command

```
%yp[<space>

<alt-s>gse

cpub use<esc>leypi::
```

1. `%` select all 3 "mod" statements
1. `yp` duplicate them
1. `[<space>` add a blank line above the 3 duplicated statements
1. `<alt-s>gse` create 3 selections for each "mod" in the duplicated statements
1. `cpub use<esc>` convert each "mod" into "pub use"
1. `ley` copy name of each module
1. `p` duplicate name of each module at the end, since each module contains a function named the same as the module
1. `i::` add a double-colon path separator between them

# Enumerate and Align

Add a new field `rank` to each object, it starts at 1 and increments and align fields to look neat.

## Before

```js
[
  { word: "a", count: 2565 },
  { word: "and", count: 1777 },
  { word: "of", count: 1331 },
  { word: "that", count: 1263 },
  { word: "to", count: 1030 },
  { word: "in", count: 1027 },
  { word: "it", count: 754 },
  { word: "as", count: 730 },
  { word: "was", count: 687 },
  { word: "you", count: 652 },
  { word: "for", count: 630 },
];
```

## After

```js
[
  { rank:  1, word: "a",    count: 2565 },
  { rank:  2, word: "and",  count: 1777 },
  { rank:  3, word: "of",   count: 1331 },
  { rank:  4, word: "that", count: 1263 },
  { rank:  5, word: "to",   count: 1030 },
  { rank:  6, word: "in",   count: 1027 },
  { rank:  7, word: "it",   count:  754 },
  { rank:  8, word: "as",   count:  730 },
  { rank:  9, word: "was",  count:  687 },
  { rank: 10, word: "you",  count:  652 },
  { rank: 11, word: "for",  count:  630 },
];
```

## Command

```
%s\{<enter>a rank: <ctrl-r>

#,<esc>%s |\d+<enter>&
```

1.  `%` selects full file
1.  Use `s` to enter select mode, which searches inside our selection and creates sub-selections based on a pattern
1.  Input `\{` and then hit `<enter>`, which will place a cursor on every "\{", creating many single-width selections
1.  `a ` to go into insert mode after the "\{"
1.  Input `rank: `
1.  `<ctrl-r>` followed by `#` inserts an increasing number for every selection starting with 1
1.  Input `,`
1.  `<esc>` goes back to normal mode
1.  Use `%s` to enter select mode again
1.  Input ` |\d+` which is a regular expression selecting all spaces and numbers, then hit `<enter>`
1.  `&` to align all selections in columns, note that the numbers get right-aligned

# Object into Array

Convert object into a list of tuples representing the field and the value.

## Before

```js
const palette = {
  apricot: "#f47868",
  lightning: "#ffcd1c",
  delta: "6f44f0",
};
```

## After

```js
const palette = [
  ["apricot", "#f47868"],
  ["lightning", "#ffcd1c"],
  ["delta", "6f44f0"],
];
```

## Command

```
jmr{[mi[s:<enter>

r,t,;vgsms[lems"
```

1. Go to the line below with `j`, this is because we need to be inside of the object for the next step.
1. `mr{[` replaces the nearest pair of curly braces "\{" with square brackets "["
1. `mi[` selects inside the entire array
1. Use `s` to enter select mode, which searches inside our selection and creates sub-selections based on a pattern
1. Input `:` and then hit `<enter>`, which will place a cursor on every ":" creating many single-width selections
1. `r,` replaces each selection with a ",". Essentially we've replaced each colon with a comma
1. `t,` moves the cursor on each line to the ending comma
1. `;` collapses the selection around each cursor into a single selection
1. `vgs` selects each line excluding the final comma
1. `ms[` surrounds each individual selection with "[" to turn it into an array. We're almost done here. We just need to transform the first item in each sub-array into a string.
1. `l` moves 1 character forward, replacing the selection with just a 1-width selection
1. `e` selects until the end of each word. Since we start at the first character and select until the end, this selects the entire word.
1. `ms"` surrounds each word with double quotes to make strings

# CSV to SQL

## Before

```csv
id 1,Item 1,cost 1,location 1
id 2,Item 2,cost 2,location 2
id 10,Item 10,cost 10,location 10
```

## After

```sql
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 1','Item 1','cost 1','Location 1');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 2','Item 2','cost 2','Location 2');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 10','Item 10','cost 10','Location 10');
```

## Command

```
%<alt-s>"yys\d<enter>

dhhbms

``x_ms(IINSERT INTO `

database<esc>

a.`table<esc>la <esc>

AVALUES (<esc>

"yPS,<enter>ms'A;<esc>Fl;~
```

1.  `%` selects full file
1.  `<alt-s>` split selection into multiple selections on newlines
1.  `"yy` yanks the selections into "y" register. We'll need it for later
1.  `s` and then input the pattern `\d` then `<enter>` which creates a selection on all digits
1.  `d` deletes the selections. Essentially we've removed all the digits.
1.  `hh` goes backwards 2 chars, important to make sure we are at the end of each word
1.  Use `b` to select till the beginning of every word, which also nicely selects all the words that there are
1.  `` ms` `` surrounds each word with a backtick
1.  `` ` `` switches all characters to lowercase
1.  `x` selects each line then use `_` to trim the trailing whitespace
1.  `ms(` surrounds each line with parentheses
1.  `I` goes into insert mode at the beginning of each line
1.  Type the following:

    ```
    INSERT INTO `database
    ```

1.  `<esc>` goes back to normal mode
1.  `a` to go into insert mode after the backtick then type:

    ```
    .`table
    ```

1.  `<esc>` goes back into normal mode, then `la` to enter insert mode just before the opening parentheses
1.  Add a space ` ` then `<esc>` to go back into normal mode again
1.  `A` goes into insert mode at the end of each line, now type:

    ```
    VALUES (
    ```

1.  Hit `<esc>` to leave insert mode. Your cursor will be at the closing parenthesis.
1.  `"yP` pastes our previously yanked items from the "y" register
1.  `S,<enter>` splits current selection into multiple selections on each comma
1.  `ms'` surrounds each item with a single quote
1.  `A;` adds a semicolon at the end of each line
1.  `<esc>` goes back to normal mode and `Fl` to place your cursor on the lowercase "l" of each "location"
1.  `;` collapses each selection into a single-width selection
1.  `~` toggles the case for each "l" into "L"

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


````````````

</details>

## [snake_case to camelCase](snake_case_to_camel_case.md)

Rename all fields to be camelCase.

<video autoplay controls loop>
  <source src="generated/snake_case_to_camel_case.mp4">
</video>


## [Text into Array](text_into_array.md)

Join newline-separated data into an array of strings

<video autoplay controls loop>
  <source src="generated/text_into_array.mp4">
</video>


## [Export from Rust Module](export_from_mod.md)

Each module contains a function, which we want to export.

<video autoplay controls loop>
  <source src="generated/export_from_mod.mp4">
</video>


## [Enumerate and Align](enumerate_and_align.md)

Add a new field `rank` to each object, it starts at 1 and increments and align fields to look neat.

<video autoplay controls loop>
  <source src="generated/enumerate_and_align.mp4">
</video>


## [Object into Array](object_into_array.md)

Convert object into a list of tuples representing the field and the value.

<video autoplay controls loop>
  <source src="generated/object_into_array.mp4">
</video>


## [CSV to SQL](csv_to_sql.md)



<video autoplay controls loop>
  <source src="generated/csv_to_sql.mp4">
</video>


## [Function into Class](function_into_class.md)

Convert 3 functions into a class with 3 methods.

<video autoplay controls loop>
  <source src="generated/function_into_class.mp4">
</video>



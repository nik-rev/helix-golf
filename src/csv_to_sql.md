# CSV to SQL

## Before

```csv
id 1,Item 1,cost 1,location 1
id 2,Item 2,cost 2,location 2
id 10,Item 10,cost 10,location 10
```

## After

```sql
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 1','Item 1','Cost 1','Location 1');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 2','Item 2','Cost 2','Location 2');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 10','Item 10','Cost 10','Location 10');
```

## Command

```
%<alt-s>"yys\d<enter>dhhbms``x_ms
(IINSERT INTO `database<esc>a.`table
<esc>la <esc>AVALUES (<esc>"yPS,<enter>
ms'A;<esc>Fl;~
```

1.  `%` selects full file
1.  `Alt` + `s` split selection into multiple selections on newlines
1.  `"yy` yanks the selections into "y" register. We'll need it for later
1.  `s` and then input the pattern `\d` then `Enter` which creates a selection on all digits
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

1.  `Escape` goes back to normal mode
1.  `a` to go into insert mode after the backtick then type:

        ```
        .`table
        ```

1.  `Escape` goes back into normal mode, then `la` to enter insert mode just before the opening parentheses
1.  Add a `Space` then `Escape` to go back into normal mode again
1.  `A` goes into insert mode at the end of each line, now type:

        ```
        VALUES (
        ```

1.  Hit `Escape` to leave insert mode. Your cursor will be at the closing parenthesis.
1.  `"yP` pastes our previously yanked items from the "y" register
1.  `S,<enter>` splits current selection into multiple selections on each comma
1.  `ms'` surrounds each item with a single quote
1.  `A;` adds a semicolon at the end of each line
1.  `Escape` goes back to normal mode and `Fl` to place your cursor on the lowercase "l" of each "location"
1.  `;` collapses each selection into a single-width selection
1.  `~` toggles the case for each "l" into "L"

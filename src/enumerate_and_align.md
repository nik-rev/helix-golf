# Enumerate and Align

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

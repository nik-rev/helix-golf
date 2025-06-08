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

r,bems"vt,ms[
```

1. Go to the line below with `j`, this is because we need to be inside of the object for the next step.
1. `mr{[` replaces the nearest pair of curly braces "\{" with square brackets "["
1. `mi[` selects inside the entire array
1. Use `s` to enter select mode, which searches inside our selection and creates sub-selections based on a pattern
1. Input `:` and then hit `<enter>`, which will place a cursor on every ":" creating many single-width selections
1. `r,` replaces each selection with a ",". Essentially we've replaced each colon with a comma
1. `be` selects the previous word on each line and moves each cursor to the end of each word
1. `ms"` surrounds each word with double quotes to make strings
1. `vt,` selects each line excluding the final comma
1. `ms[` surrounds each individual selection with "[" to turn it into an array

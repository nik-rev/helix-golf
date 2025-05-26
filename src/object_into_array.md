Refatoring examples for the Helix Editor

_For all examples, we'll assume that your cursor is on the very first character._

# Object into Array

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

## Preview

<video controls>
  <source src="generated/object_into_array.mp4" type="video/mp4">
</video>

## Command

```
jmr{[mi[s:<enter>r,t,;vgsms[lems"
```

1. Go to the line below with `j`, this is because we need to be inside of the object for the next step.
1. `mr{[` replaces the nearest pair of curly braces "\{" with square brackets "["
1. `mi[` selects inside the entire array
1. Use `s` to enter select mode, which searches inside our selection and creates sub-selections based on a pattern
1. Input `:` and then hit `Enter`, which will place a cursor on every ":" creating many single-width selections
1. `r,` replaces each selection with a ",". Essentially we've replaced each colon with a comma
1. `t,` moves the cursor on each line to the ending comma
1. `;` collapses the selection around each cursor into a single selection
1. `vgs` selects each line excluding the final comma
1. `ms[` surrounds each individual selection with "[" to turn it into an array. We're almost done here. We just need to transform the first item in each sub-array into a string.
1. `l` moves 1 character forward, replacing the selection with just a 1-width selection
1. `e` selects until the end of each word. Since we start at the first character and select until the end, this selects the entire word.
1. `ms"` surrounds each word with double quotes to make strings

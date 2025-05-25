# Text into Array

## Before

```
Hello
This
Is
Helix
```

## After

```js
["Hello", "This", "Is", "Helix"];
```

## Command

```
%<A-s>ms"<A-J>i,<esc>xms ms[
```

1. `%` selects full file
1. `<A-s>` split selection into multiple selections on newlines
1. `ms"` surrounds each word with `"`'s
1. `<A-J>i,` join lines inside selection, select the inserted space, and insert `,`'s
1. `<esc>xms ` enter normal mode, select line and surround by spaces
1. `ms[` surround by `[]`

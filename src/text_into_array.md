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

## Preview

<video controls>
  <source src="generated/text_into_array.mp4" type="video/mp4">
</video>

## Command

```
%<alt-s>ms"<alt-J>i,<esc>xms ms[
```

1. `%` selects full file
1. `<alt-s>` split selection into multiple selections on newlines
1. `ms"` surrounds each word with `"`'s
1. `<alt-J>i,` join lines inside selection, select the inserted space, and insert `,`'s
1. `<esc>xms[` surround by `[]`

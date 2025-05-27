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

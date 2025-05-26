# Helix Golf

Helix is _very_ good at editing text and this website has examples of how I've refactored some snippets of code using it.

[https://nik-rev.github.io/helix-golf](https://nik-rev.github.io/helix-golf/)

## Contributing

To add a new example create `src/your_example.md` using the following template.

````md
# Title

Made `h` capital and added exclamation mark.

## Before

```
hello world
```

## After

```
Hello world!
```

## Preview

![preview](generated/your_example.mp4)

## Command

```
~A!
```

1. `~` changes case of the selection
1. `A` go to end and enter insert mode
1. `!` write the exclamation mark
````

Notes:

- The concatenation of code blocks in the description list needs to be the same as the
  code block after `## Command`.

  So in here:

  ````md
  1. `~` changes case of the selection
  1. Go to end and enter insert mode:

     ```
     A
     ```

  1. `!` write the exclamation mark
  ````

  Concatenating all of the code blocks gives `~A!`, which is the same as the code block after `## Command` title.

- It'll also need to be included in [`SUMMARY.md`](src/SUMMARY.md)

- The exact structure is enforced by the script `cargo generate_demos`, which will provide a friendly error message if the structure is wrong.

### Demos

Generate the demo for each example by running the following command in the project root directory:

```sh
cargo generate_demos
```

You will need to install [vhs](https://github.com/charmbracelet/vhs) for it to work.

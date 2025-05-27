# Helix Golf

Helix is _very_ good at editing text and this website has examples of how I've refactored some snippets of code using it.

[https://nik-rev.github.io/helix-golf](https://nik-rev.github.io/helix-golf/)

## Contributing

If you want to suggest a new example, make an issue and I'll add it.

---

If you want to add a new example yourself: create `src/your_example.md` using the following template.

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

## Command

```
~A!
```

1. `~` changes case of the selection
1. `A` go to end and enter insert mode
1. `!` write the exclamation mark
````

### Dependencies

- [Helix](https://docs.helix-editor.com/install.html)
- [Rust](https://www.rust-lang.org/tools/install)
- [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)
- [VHS](https://github.com/charmbracelet/vhs?tab=readme-ov-file#installation) to generate the demo files and test examples for correctness

If you don't want to install them but still would like to contribute, you can edit the markdown example files in the [`src/`](src/) folder, send a pull request and the GitHub CI will automatically test your PR.

### Validate

Verify that your example is correctly structured by running the following command in the project root:

```sh
cargo validate
```

### Generate Demos

The demos for each example are generated and tested by running the following command:

```sh
cargo generate-demos
```

You can specify exactly which demos to generate:

```sh
cargo generate-demos export_from_mod
```

### Running locally

You can run the website locally by running:

```sh
mdbook serve
```

It will be available on `http://localhost:3000`.

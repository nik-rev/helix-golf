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

### Validate

Verify that your example is correctly structured by running the following command in the project root (requires [installing Rust](https://www.rust-lang.org/tools/install)):

```sh
cargo validate
```

### Generate Demos

The demos for each example are generated and tested by running the following command, which requires [installing VHS](https://github.com/charmbracelet/vhs?tab=readme-ov-file#installation):

```sh
cargo generate_demos
```

### Running locally

You can build the website by running the following command:

```sh
mdbook serve
```

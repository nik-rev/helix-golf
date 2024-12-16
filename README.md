# Helix Golf

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

Examples of efficiently refactoring text in the [Helix Editor](https://helix-editor.com/).

## Contributing

### Adding new examples

To add an entry, copy-paste the following template into the [`index.mdx`](https://github.com/NikitaRevenco/helix-golf/edit/main/src/content/docs/index.mdx) file:

````mdx
#### A concise title

An optional description.

##### Before

```file extension
input file
```

##### After

```file extension
output file
```

##### Command

<details>

<summary>`insert command here`</summary>

1. Steps taken
1. To perform the
1. Transformation

</details>
````

### Running locally

1. Clone this repository
1. `pnpm install`
1. `pnpm dev`

# Export from Rust Module

Each module contains a function, which we want to export.

## Before

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;
```

## After

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;

pub use generate_demos::generate_demos;
pub use mdbook_preprocessor::mdbook_preprocessor;
pub use validate::validate;
```

## Command

```
%yp[<space>

<alt-s>gse

cpub use<esc>leypi::
```

1. `%` select all 3 "mod" statements
1. `yp` duplicate them
1. `[<space>` add a blank line above the 3 duplicated statements
1. `<alt-s>gse` create 3 selections for each "mod" in the duplicated statements
1. `cpub use<esc>` convert each "mod" into "pub use"
1. `ley` copy name of each module
1. `p` duplicate name of each module at the end, since each module contains a function named the same as the module
1. `i::` add a double-colon path separator between them

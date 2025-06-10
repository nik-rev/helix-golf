# Reverse Golf Example

Switch the "Before" and "After" cases for a Helix Golf example.

## Before

````md
# snake_case to camelCase

Rename all fields to be camelCase.

## Before

```js
const user_profile = {first_name: "John"};
```

## After

```js
const userProfile = {firstName: "John"};
```
````

## After

````md
# camelCase to snake_case

Rename all fields to be snake_case.

## Before

```js
const userProfile = {firstName: "John"};
```

## After

```js
const user_profile = {first_name: "John"};
```
````

## Command

```
ebyxb*Rv""Nn<alt-)>

%s`+j<enter>f;<alt-(>
```

1. `eb` selects the next word and trims whitespace
1. `y` copies the selected word and yanks it into the " (double quote) register
1. `xb` selects the last word in the line
1. `*` sets the current selection as the search pattern
1. `R` replaces the selected word with the copied selection from earlier
1. `v` enters select mode
1. `""` selects the " (double quote) register. Pressing `N` will add a new selection at the previous occurrence of the word saved to the register
1. `n` adds a new selection at the next occurrence of the search pattern assigned earlier
1. `<alt-)>` rotates the contents of the selections forward
1. `%` selects the entire contents of the file
1. `s` brings up a prompt to select sub-selections by a given regex. We type in `` `+j `` for the regex and select all matches with `<enter>` 
1. Since we're still in select mode, typing `f;` moves each cursor to select up until (and including) the next occurrence of a semicolon
1. `<alt-(>` rotates the contents of the selections backwards 

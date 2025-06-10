# Alternating Caps

Modify the text to alternate between lowercase and uppercase letters

## Before

```
Consider bringing your keyboard to the doctor for a brief checkup
if it's typing in alternating caps. It may be a bad case of
sticky keys.
```

## After

```
cOnSiDeR bRiNgInG yOuR kEyBoArD tO tHe DoCtOr FoR a BrIeF cHeCkUp
If It'S tYpInG iN aLtErNaTiNg CaPs. It MaY bE a BaD cAsE oF
sTiCkY kEyS.
```

## Command

```
%`s[a-z][^a-z]*[a-z]<enter>;~
```

1. `%` selects the whole file
1. `` ` `` changes all selected letters to lowercase
1. `s` brings up a prompt to select sub-selections by a given regex.
1. Typing `[a-z][^a-z]*[a-z]` will match all consecutive letter pairs, even when they are separated by things like spaces or newline characters. The "[a-z]" parts of the regex match any single lowercase letter, and "[^a-z]*" matches zero or more occurrences of any character that isn't a lowercase letter.
1. Hit `<enter>` to select all regex matches
1. `;` collapses each selection into a single-width selection
1. `~` toggles the case for each selected letter

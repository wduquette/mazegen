# rand -- Random Number Generator

**Syntax: rand *subcommand* ?*args*...?**

The **rand** command provides access to the **rand** crate's `thread_rng`.  It has the
following subcommands.

| Subcommand                  | Description |
| ----------------------------| ----------- |
| [rand bool](#rand-bool)     | Generates a random boolean |
| [rand range](#rand-range)   | Generates an integer from a range |
| [rand sample](#rand-sample) | Selects an item randomly from a list |

## rand bool
---
**Syntax: rand bool ?*prob*?**

Returns either a 1 or 0, both with probability 0.5.  If given, *prob* must be a number
between 0.0 and 1.0 indicating the probability of getting a 1.

TODO: Rejects *prob* if it is exactly 0.0 or 1.0.  That might be a nuisance.

## rand range
---
**Syntax: rand range ?*start*? *end***

Randomly selects and returns a number in the range *start* to (*end* - 1).  The *start* value
defaults to 0.

## rand sample
---
**Syntax: rand sample *list...***

Randomly selects and returns an element from a list.  The list can be provided as a single value,
or as multiple elements on the command line, i.e., the two calls shown below both return a
random letter "a", "b", "c", "d", or "e".

```tcl
rand sample a b c d e

set list [list a b c d e]
rand sample $list
```

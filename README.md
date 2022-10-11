# Simple Turing machine

Verify Turing machine computations in Simplicity.

It is easy to verify that a Boolean function accepts (returns "true" upon) a given input. This verification is strictly easier than computing the output of a function. Simplicity is Turing-incomplete and yet it can verify any computable function in this way.

## Running

```
cargo run --example trivial
cargo run --example two_beavers
...
```

### Input

The computation is given as witness data and consists of a sequence of configurations. A configuration consists of the current state, tape and index (pointer).

Feel free to change the inputs of the given examples to experiment.

### Output

If the computation is valid, then the Simplicity program returns nothing (NOP). Otherwise, the program reaches a so-called pruned branch with one of the following error codes:

- 00... → invalid state
- 01... → invalid index
- 02... → invalid tape
- 03... → invalid initial state
- 04... → invalid accepting state

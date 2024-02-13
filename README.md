# cropduster

A genetic algorithm that finds the best arrangement of crops in Minecraft,
written in Rust.

Knowing that crops grow fastest when crops of the same type are not diagonally
adjacent, an arrangement of crops for a given rectangle is scored based on how
many crops meet those criteria, and then the best crops survive each iteration
unchanged, the worst "die" (are replaced by random arrangements), and
inbetweens are mutated (one extra change added).

## Example Usage

See `src/main.rs` to see and edit the parameters, then do:

```
$ cargo run --release
```

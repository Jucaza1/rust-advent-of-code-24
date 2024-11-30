# Advent of Code 2023

This repository includes my [Rust](https://www.rust-lang.org/) code for the [Advent of Code 2024](https://adventofcode.com/2024).

Ah, it's that time of the year again!

*If you want to solve the puzzles yourself first, do so, **then** look for the solution to compare.*
Do **not** cheat yourself out of a great time and learning experience by just copying code!

**The puzzles are solved in examples rather than all in the same entrypoint**
I'll update this repository now and then to include all my solutions whenever I've got time to properly comment and upload it.

This is mostly for those that are curious.
If you want to look at the code, experiment with it, change it, etc. be my guest.

## Running from command line

To run the code for any given day, use the following line (replacing `1` with the number of the desired day):

```bash
cargo run --example day-XX-part-XX
```
or run in watch mode

```bash
cargo watch -q -c -x "cargo run -q --example day-XX-part-XX"
```
`-q` for quiet.
`-c` clear the terminal.
`-x` execute the following string.

As an alternative, you can also compile the executables explicitly, using `build` rather than `run`:

```bash
cargo build
```

Inputs are expected to be in the `examples/day-XX-part-XX/` sub folder relative to the current working directory.
Each input.txt and its input-test.txt are both in the same folder as the example named main.rs inside its onw folder.


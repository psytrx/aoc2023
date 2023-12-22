# aoc2023

My Advent of Code 2023 solutions, this year in Rust.

## Constraints

Some self-imposed constraints, because I participate to learn, not to compete.

- No use of regular expressions
  - (unless the problem is targeted to need them)
  - My prediction: A complete parsing problem at day 10,
    a full interpreter at day 20
- Optimize for performance
  - Requires me to learn more about the inner workings, the use of profilers,
    struct alignment, etc.
  - Requires thoughtful balancing between performance improvements and readability
- Full memory- and runtime-safety. No use of `unsafe`.
  - Each and every possible error case must be handled and passed up to `main`,
    even input parsing errors
  - Might be a bit verbose at times, let's see...

## Usage

By default, only unsolved problems are run.

To detect whether a problem has been solved,
the program checks for a file in the `/answers` directory.

By default, solutions are _not_ printed out.

`--validate` is used to test against regressions.

```bash
Usage: aoc2023 [OPTIONS]

Options:
  -f, --force-all       Runs all solutions, even if they have been solved already
  -n <N>                Number of times to run all solutions. Used for benchmarking [default: 1]
  -v, --validate        Validates the solutions agains the answers in the /answers directory
      --show-solutions  Prints solutions to stdout
  -h, --help            Print help

```

### Scripts

```bash
# Runs all solutions and validates them.
# Useful for taking performance snapshots.
./run-all.sh

# Runs all unsolved solutions and watches the `/src` directory
# Useful during problem solving.
./watch.sh

# Runs all solutions and takes a profile for samply.
# Requires samply to be installed: https://github.com/mstange/samply
./samply.sh
```

## Latest Problem Runtimes

```log
[2023-12-22T17:57:55Z DEBUG aoc2023] day 01/part 1 took 0.08ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 01/part 2 took 0.12ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 02/part 1 took 0.05ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 02/part 2 took 0.04ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 03/part 1 took 0.1ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 03/part 2 took 0.06ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 04/part 1 took 0.1ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 04/part 2 took 0.1ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 05/part 1 took 0.02ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 05/part 2 took 0.06ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 06/part 1 took 0ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 06/part 2 took 0ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 07/part 1 took 0.09ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 07/part 2 took 0.09ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 08/part 1 took 0.12ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 08/part 2 took 0.29ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 09/part 1 took 0.24ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 09/part 2 took 0.26ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 10/part 1 took 0.3ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 10/part 2 took 0.33ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 11/part 1 took 0.13ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 11/part 2 took 0.08ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 12/part 1 took 0.32ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 12/part 2 took 1.04ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 13/part 1 took 0.09ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 13/part 2 took 0.1ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 14/part 1 took 0.02ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 14/part 2 took 4.84ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 15/part 1 took 0.05ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 15/part 2 took 0.31ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 16/part 1 took 0.18ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 16/part 2 took 6.34ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 17/part 1 took 4.51ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 17/part 2 took 7.4ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 18/part 1 took 0.06ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 18/part 2 took 0.11ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 19/part 1 took 0.18ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 19/part 2 took 0.18ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 20/part 1 took 3.53ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 20/part 2 took 0ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 21/part 1 took 0.13ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 21/part 2 took 0ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 22/part 1 took 3.33ms
[2023-12-22T17:57:55Z DEBUG aoc2023] day 22/part 2 took 5.7ms

[2023-12-22T17:57:55Z DEBUG aoc2023] Total duration: 41.305075ms
```

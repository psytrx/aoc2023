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

## Latest Problem Runtimes

```log
[2023-12-08T13:46:01Z DEBUG aoc2023] day 01/part 1 took 0.09ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 01/part 2 took 0.14ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 02/part 1 took 0.05ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 02/part 2 took 0.04ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 03/part 1 took 0.11ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 03/part 2 took 0.07ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 04/part 1 took 0.12ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 04/part 2 took 0.13ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 05/part 1 took 0.03ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 05/part 2 took 0.07ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 06/part 1 took 0ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 06/part 2 took 0ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 07/part 1 took 0.17ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 07/part 2 took 0.14ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 08/part 1 took 0.13ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 08/part 2 took 0.29ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 09/part 1 took 0ms
[2023-12-08T13:46:01Z DEBUG aoc2023] day 09/part 2 took 0ms

[2023-12-08T13:46:01Z DEBUG aoc2023] Total duration: 1.657311ms
```

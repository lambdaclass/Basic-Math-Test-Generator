# Basic-Math-Test-Generator

## About

Generates a basic math test in your terminal. This is mainly geared towards children to help them practice. The numbers used in each problem are generated at random. The difficulty can be adjusted using the `--difficulty` flag. The types of operations used in the generated problem can also be adjusted using the flags.

## Motivation

I'm a father and engineer. I'm currently learning Rust and my children love solving math problems. So it

## Installation

```shell
cargo install basic_math_problem_generator
```

## Generating Tests

### Basic Addition

```shell
basic_math_problem_generator
```

### Adjusting Number of problems

Default number of problems is 10. This can be adjusted using the `--problems` flag.

```shell
basic_math_problem_generator --problems 20
```

### Adjusting Difficulty

Default diffulty is level 1. The diffulty of the test can be adjusted using the `--difficulty` flag.
- Level 1 - Problems use numbers less than 10.
- Level 2 - Problems use numbers less than 100.
- Level 3 - Problems use numbers less than 1000.

```shell
basic_math_problem_generator --difficulty 2
```

### Adjusting Operations

Default is only addition problems.

- `--no-addition` flag has to be used in conjunction with one of the flags below. Removes addition problems.
- `--subtraction` flag adds subtraction problems.
- `--multiplication` flag adds multiplication problems.

For Multiplication only.
```shell
basic_math_problem_generator --no-addition --multiplication
```

For a mixed of all operations.
```shell
basic_math_problem_generator --subtraction --multiplication --division
```
[![ci](https://github.com/CCecilia/Basic-Math-Test-Generator/actions/workflows/ci.yml/badge.svg)](https://github.com/CCecilia/Basic-Math-Test-Generator/actions/workflows/ci.yml)

# Basic-Math-Test-Generator

## About

Generates a basic math test in your terminal or generated into a text file with a answer key if you want a paper version. This is mainly geared towards children to help them practice. The numbers used in each problem are generated at random. The difficulty can be adjusted using the `--difficulty` flag. The types of operations used in the generated problem can also be adjusted using the flags.

## Motivation

I'm a father and engineer. I'm currently learning Rust and my children love solving math problems. So it only makes sense.

## Installation

```shell
cargo install basic_math_problem_generator
```

## Generating Tests

### Basic Addition

```shell
math-test
```

### Adjusting Number of problems

Default number of problems is 10. This can be adjusted using the `--problems` flag.

```shell
math-test --problems 20
```

### Adjusting Difficulty

Default diffulty is level 1. The diffulty of the test can be adjusted using the `--difficulty` flag.
- Level 1 - Problems use numbers less than 10.
- Level 2 - Problems use numbers less than 100.
- Level 3 - Problems use numbers less than 1000.

```shell
math-test --difficulty 2
```

### Adjusting Operations

Default is only addition problems.

- `--no-addition` flag has to be used in conjunction with one of the flags below. Removes addition problems.
- `--subtraction` flag adds subtraction problems.
- `--multiplication` flag adds multiplication problems.

For Multiplication only.
```shell
math-test --no-addition --multiplication
```

For a mixed of all operations.
```shell
math-test --subtraction --multiplication
```

### Paper Test

To create a paper test version with a answer key use the following command below. This will create directory name `Math-Test` in whatever directory you call the command from and will contain two files `questions.txt` and `answers.txt`. The problems will be numbered to easily correlate the  answers to the questions.

```shell
math-test --paper-test
```
use clap::Parser;
use math_problem::{MathProblem, Operation};
use math_test::MathTest;
use std::{io::stdin, process::exit};

mod math_problem;
mod math_test;

#[derive(Parser)]
#[command(name = "MathProblemGenerator")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Generates random basic problem for practicing", long_about = None)]
struct MathProblemGeneratorInterface {
    #[arg(long, default_value_t = 10, help = "Number of problems to generate")]
    problems: u32,
    #[arg(
        long,
        default_value_t = 1,
        help = "Difficulty 1-3, 1: numbers < 10, 2: numbers < 100, 3: numbers < 1000"
    )]
    difficulty: u32,
    #[arg(short, long, default_value_t = false)]
    no_addition: bool,
    #[arg(short, long, default_value_t = false)]
    subtraction: bool,
    #[arg(short, long, default_value_t = false)]
    multiplication: bool,
    #[arg(short, long, default_value_t = false)]
    division: bool,
}

fn main() {
    let cli_args = MathProblemGeneratorInterface::parse();
    let number_of_problems_to_generate = cli_args.problems;
    let mut math_test = MathTest::new();

    for _i in 0..number_of_problems_to_generate {
        let mut allowed_operations: Vec<Operation> = vec![];
        if !cli_args.no_addition {
            allowed_operations.push(Operation::Addition('+'));
        }

        if cli_args.subtraction {
            allowed_operations.push(Operation::Subtraction('-'));
        }

        if cli_args.multiplication {
            allowed_operations.push(Operation::Multiplication('*'));
        }

        if cli_args.division {
            allowed_operations.push(Operation::Division('/'));
        }

        if allowed_operations.is_empty() {
            println!(
                "Please choose atleast one operation in conjunction with the --no_addition flag."
            );
            exit(0);
        }

        let mut current_problem = MathProblem::new(cli_args.difficulty, &allowed_operations);
        let mut user_input = String::new();

        println!("\nProblem:  {}", current_problem.ui_string);
        if let Ok(_) = stdin().read_line(&mut user_input) {
            if let Ok(user_answer) = user_input.trim().parse::<u32>() {
                current_problem.user_answer = Some(user_answer);
                if user_answer == current_problem.expected_answer {
                    current_problem.user_correct = Some(true);
                } else {
                    current_problem.user_correct = Some(false);
                }
                math_test.problems.push(current_problem);
            };
        };
    }

    println!("{}", math_test.get_results())
}

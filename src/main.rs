use clap::Parser;
use math_problem::{MathProblem, Operation};
use math_test::MathTest;
use std::{
    env::current_dir,
    fs,
    io::{self, BufWriter, Write},
    path,
    process::exit,
};
use utils_lib::open_file;

mod math_problem;
mod math_test;
mod utils_lib;

#[derive(Parser)]
#[command(name = "MathProblemGenerator")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Generates random basic problem for practicing", long_about = None)]
struct MathProblemGeneratorInterface {
    #[arg(long, default_value_t = 10, help = "Number of problems to generate")]
    problems: u32,
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "Difficulty 1-3, 1: numbers < 10, 2: numbers < 100, 3: numbers < 1000"
    )]
    difficulty: u32,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Exclude addition problems"
    )]
    no_addition: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Add subtraction problems"
    )]
    subtraction: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Add multiplication problems"
    )]
    multiplication: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "Creates a file with math problem and another with the answer key"
    )]
    paper_test: bool,
}

fn main() {
    let cli_args = MathProblemGeneratorInterface::parse();
    let number_of_problems_to_generate = cli_args.problems;
    let mut math_test = MathTest::new(cli_args.difficulty);
    let mut allowed_operations: Vec<Operation> = vec![];
    let mut question_file_write_buffer: Option<BufWriter<fs::File>> = None;
    let mut answer_file_write_buffer: Option<BufWriter<fs::File>> = None;

    if !cli_args.no_addition {
        allowed_operations.push(Operation::Addition('+'));
    }

    if cli_args.subtraction {
        allowed_operations.push(Operation::Subtraction('-'));
    }

    if cli_args.multiplication {
        allowed_operations.push(Operation::Multiplication('*'));
    }

    if allowed_operations.is_empty() {
        println!("Please choose atleast one operation in conjunction with the --no_addition flag.");
        exit(1);
    }

    if cli_args.paper_test {
        match current_dir() {
            Ok(cwd) => {
                println!("cwd {:?}", cwd);
                let math_test_dir = cwd.join(path::Path::new("math-problems"));

                println!("Math testDir {:?}", math_test_dir);

                if !math_test_dir.exists() {
                    let create_dir_result = fs::create_dir(math_test_dir.as_path());
                    if let Err(e) = create_dir_result {
                        eprintln!("Failed to create dir for question and answer files\n{}", e);
                        exit(1);
                    }
                }

                if let Ok(file) = open_file(math_test_dir.join(path::Path::new("questions.txt"))) {
                    question_file_write_buffer = Some(BufWriter::new(file));
                }

                if let Ok(file) = open_file(math_test_dir.join(path::Path::new("answers.txt"))) {
                    answer_file_write_buffer = Some(BufWriter::new(file));
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to create dir for question and answer files\n{}",
                    e.to_string()
                );
                exit(1)
            }
        }

        // if let Some(question_file_path) = &question_file_path {
        question_file_write_buffer = match question_file_write_buffer {
            Some(mut writer) => {
                let line = "Name:_________________________________________________\n\n\n\n";
                if let Err(e) = writer.write_all(line.as_bytes()) {
                    eprintln!("{}", e);
                }
                Some(writer)
            }
            None => None,
        };

        answer_file_write_buffer = match answer_file_write_buffer {
            Some(mut writer) => {
                let line = "Answer Key\n\n\n\n";
                if let Err(e) = writer.write_all(line.as_bytes()) {
                    eprintln!("{}", e);
                }
                Some(writer)
            }
            None => None,
        };
    }

    for i in 0..number_of_problems_to_generate {
        let problem_number = i + 1;
        let mut current_problem =
            MathProblem::new(cli_args.difficulty, &allowed_operations, problem_number);
        let mut user_input = String::new();

        if cli_args.paper_test {
            question_file_write_buffer = match question_file_write_buffer {
                Some(mut writer) => {
                    let line = format!(
                        "{}. {}\n\n\n\n",
                        &current_problem.problem_number.to_string(),
                        &current_problem.ui_string
                    );
                    if let Err(e) = writer.write_all(line.as_bytes()) {
                        eprintln!("{}", e);
                    }
                    Some(writer)
                }
                None => None,
            };

            answer_file_write_buffer = match answer_file_write_buffer {
                Some(mut writer) => {
                    let line = format!(
                        "{}. {}\n\n",
                        &current_problem.problem_number.to_string(),
                        &current_problem.expected_answer
                    );
                    if let Err(e) = writer.write_all(line.as_bytes()) {
                        eprintln!("{}", e);
                    }
                    Some(writer)
                }
                None => None,
            };
        } else {
            println!("\nProblem:  {}", current_problem.ui_string);
            if io::stdin().read_line(&mut user_input).is_ok() {
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
    }

    if !cli_args.paper_test {
        println!("{}", math_test.get_results())
    }
}

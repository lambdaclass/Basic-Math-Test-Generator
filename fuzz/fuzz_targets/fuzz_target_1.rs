#![no_main]
use basic_math_problem_generator::{Operation, MathProblem};
use libfuzzer_sys::fuzz_target;

fn build_u32_from(data: &[u8], offset: usize) -> u32 {
    ((data[offset] as u32) << 0) | ((data[offset + 1] as u32) << 8) | ((data[offset + 2] as u32) << 16) | ((data[offset + 3] as u32) << 24)
}

fuzz_target!(|data: [u8;8]| {
    // fuzzed code goes here
    let allowed_operations = vec![Operation::Addition('+'), Operation::Subtraction('-'), Operation::Multiplication('*')];
    let difficulty : u32 = build_u32_from(&data, 0);
    let problem_number : u32 = build_u32_from(&data, 4);
    let problem_suffix = "?";

    let _ = MathProblem::new(difficulty, &allowed_operations, problem_number, problem_suffix);
});

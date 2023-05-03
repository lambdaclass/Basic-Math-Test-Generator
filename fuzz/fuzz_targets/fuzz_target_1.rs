#![no_main]

use libfuzzer_sys::fuzz_target;

use basic_math_problem_generator::MathProblem;
use basic_math_problem_generator::Operation;

// Panics cuando allowed_operations esta vacio: panicked at 'cannot sample empty range'
// Panics linea 48 math_problem.rs: thread 'main' panicked at 'attempt to subtract with overflow'
fuzz_target!(|data: &[u8]| {
    if data.len() > 17 {
        let diff_bytes = [data[0], data[1], data[2], data[3]];
        let difficulty = u32::from_be_bytes(diff_bytes);
        let problem_number_bytes = [data[4], data[5], data[6], data[7]];
        let problem_number = u32::from_be_bytes(problem_number_bytes);
        let mut operations = Vec::new();
        for i in 8..16 {
            operations.push(Operation::Addition(data[i] as char));
        }
        for i in 8..16 {
            operations.push(Operation::Subtraction(data[i] as char));
        }
        for i in 8..16 {
            operations.push(Operation::Multiplication(data[i] as char));
        }

        MathProblem::new(difficulty, &operations, problem_number, "hola");
    }
});

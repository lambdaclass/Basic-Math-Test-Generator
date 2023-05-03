#![no_main]

use libfuzzer_sys::fuzz_target;

use basic_math_problem_generator::MathProblem;

// Panics cuando allowed_operations esta vacio: panicked at 'cannot sample empty range'
fuzz_target!(|data: &[u8]| {
    if data.len() > 3 {
        let diff_bytes = [data[0], data[1], data[2], data[3]];
        let difficulty = u32::from_be_bytes(diff_bytes);
        let problem_number = u32::from_be_bytes(diff_bytes);
        MathProblem::new(difficulty, &vec![], problem_number, "hola");
    }
});

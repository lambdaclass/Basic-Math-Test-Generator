use crate::math_problem::{MathProblem, Operation};

pub struct MathTest {
    pub problems: Vec<MathProblem>,
}

impl MathTest {
    pub fn new() -> Self {
        let problems = vec![];

        MathTest { problems }
    }

    pub fn get_results(&self) -> String {
        let mut results = String::from("###################### RESULTS ######################\n");
        let total_questions = self.problems.len();
        let mut total_incorrect = 0;
        for i in 0..total_questions {
            let problem = &self.problems[i];
            if let Some(outcome) = problem.user_correct {
                if outcome {
                    match problem.operation {
                        Operation::Addition(ref operation_char) => results.push_str(
                            format!(
                                "\nCORRECT {} {} {} = {}\n",
                                problem.first_number,
                                operation_char,
                                problem.second_number,
                                problem.expected_answer
                            )
                            .as_str(),
                        ),
                        Operation::Subtraction(ref operation_char) => results.push_str(
                            format!(
                                "\nCORRECT {} {} {} = {}\n",
                                problem.first_number,
                                operation_char,
                                problem.second_number,
                                problem.expected_answer
                            )
                            .as_str(),
                        ),
                        Operation::Multiplication(ref operation_char) => results.push_str(
                            format!(
                                "\nCORRECT {} {} {} = {}\n",
                                problem.first_number,
                                operation_char,
                                problem.second_number,
                                problem.expected_answer
                            )
                            .as_str(),
                        ),
                        Operation::Division(ref operation_char) => results.push_str(
                            format!(
                                "\nCORRECT {} {} {} = {}\n",
                                problem.first_number,
                                operation_char,
                                problem.second_number,
                                problem.expected_answer
                            )
                            .as_str(),
                        ),
                    }
                } else {
                    total_incorrect += 1;
                    match problem.operation {
                        Operation::Addition(ref operation_char) => {
                            if let Some(user_answer) = problem.user_answer {
                                results.push_str(
                                    format!(
                                        "\nINCORRECT\nUser Answer: {fn} {op} {sn} = {ua}\nExpected: {fn} {op} {sn} = {ea}\n",
                                        fn=problem.first_number,
                                        op=operation_char,
                                        sn=problem.second_number,
                                        ua=user_answer,
                                        ea=problem.expected_answer
                                    )
                                    .as_str(),
                                )
                            }
                        }
                        Operation::Subtraction(ref operation_char) => {
                            if let Some(user_answer) = problem.user_answer {
                                results.push_str(
                                    format!(
                                        "\nINCORRECT\nUser Answer: {fn} {op} {sn} = {ua}\nExpected: {fn} {op} {sn} = {ea}\n",
                                        fn=problem.first_number,
                                        op=operation_char,
                                        sn=problem.second_number,
                                        ua=user_answer,
                                        ea=problem.expected_answer
                                    )
                                    .as_str(),
                                )
                            }
                        }
                        Operation::Multiplication(ref operation_char) => {
                            if let Some(user_answer) = problem.user_answer {
                                results.push_str(
                                    format!(
                                        "\nINCORRECT\nUser Answer: {fn} {op} {sn} = {ua}\nExpected: {fn} {op} {sn} = {ea}\n",
                                        fn=problem.first_number,
                                        op=operation_char,
                                        sn=problem.second_number,
                                        ua=user_answer,
                                        ea=problem.expected_answer
                                    )
                                    .as_str(),
                                )
                            }
                        }
                        Operation::Division(ref operation_char) => {
                            if let Some(user_answer) = problem.user_answer {
                                results.push_str(
                                    format!(
                                        "\nINCORRECT\nUser Answer: {fn} {op} {sn} = {ua}\nExpected: {fn} {op} {sn} = {ea}\n",
                                        fn=problem.first_number,
                                        op=operation_char,
                                        sn=problem.second_number,
                                        ua=user_answer,
                                        ea=problem.expected_answer
                                    )
                                    .as_str(),
                                )
                            }
                        }
                    }
                }
            }
        }

        results.push_str(
            format!(
                "\nTotal Questions: {}\nTotal Incorrect: {}\nScore: {}",
                total_questions,
                total_incorrect,
                100 - (100 / total_questions * total_incorrect)
            )
            .as_str(),
        );
        results
    }
}

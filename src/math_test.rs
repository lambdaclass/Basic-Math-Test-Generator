use crate::math_problem::{MathProblem, Operation};

pub struct MathTest {
    pub problems: Vec<MathProblem>,
}

impl MathTest {
    pub fn new() -> Self {
        let problems = vec![];

        MathTest { problems }
    }

    fn add_correct_answer_to_results(
        &self,
        mut results: String,
        first_number: &u32,
        operator: &char,
        second_number: &u32,
        expected_answer: &u32,
    ) -> String {
        results.push_str(
            format!(
                "\nCORRECT {} {} {} = {}\n",
                first_number, operator, second_number, expected_answer
            )
            .as_str(),
        );

        results
    }

    fn add_incorrect_answer_to_results(
        &self,
        mut results: String,
        first_number: &u32,
        operator: &char,
        second_number: &u32,
        expected_answer: &u32,
        user_answer: &u32,
    ) -> String {
        results.push_str(
            format!(
                "\nINCORRECT\nUser Answer: {fn} {op} {sn} = {ua}\nExpected: {fn} {op} {sn} = {ea}\n",
                fn=first_number,
                op=operator,
                sn=second_number,
                ua=user_answer,
                ea=expected_answer
            )
            .as_str(),
        );

        results
    }

    fn add_score_to_results(
        &self,
        mut results: String,
        total_questions: &usize,
        total_incorrect: &usize,
    ) -> String {
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

    pub fn get_results(&mut self) -> String {
        let total_questions = self.problems.len();
        let mut total_incorrect = 0;
        let mut results = String::from("###################### RESULTS ######################\n");

        for i in 0..total_questions {
            let problem = &self.problems[i];

            if let Some(outcome) = problem.user_correct {
                if outcome {
                    results = match problem.operation {
                        Operation::Addition(ref operator) => self.add_correct_answer_to_results(
                            results,
                            &problem.first_number,
                            &operator,
                            &problem.second_number,
                            &problem.expected_answer,
                        ),
                        Operation::Subtraction(ref operator) => self.add_correct_answer_to_results(
                            results,
                            &problem.first_number,
                            &operator,
                            &problem.second_number,
                            &problem.expected_answer,
                        ),
                        Operation::Multiplication(ref operator) => self
                            .add_correct_answer_to_results(
                                results,
                                &problem.first_number,
                                &operator,
                                &problem.second_number,
                                &problem.expected_answer,
                            ),
                        Operation::Division(ref operator) => self.add_correct_answer_to_results(
                            results,
                            &problem.first_number,
                            &operator,
                            &problem.second_number,
                            &problem.expected_answer,
                        ),
                    }
                } else {
                    total_incorrect += 1;
                    if let Some(user_answer) = problem.user_answer {
                        results = match problem.operation {
                            Operation::Addition(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    &operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                            Operation::Subtraction(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    &operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                            Operation::Multiplication(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    &operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                            Operation::Division(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    &operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                        }
                    }
                }
            }
        }

        results = self.add_score_to_results(results, &total_questions, &total_incorrect);

        results
    }
}

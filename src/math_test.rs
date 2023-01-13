use crate::math_problem::{MathProblem, Operation};
use std::time::Instant;

pub struct MathTest {
    pub problems: Vec<MathProblem>,
    pub difficulty: u32,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

impl MathTest {
    pub fn new(difficulty: u32) -> Self {
        let problems = vec![];
        let start_time = Instant::now();

        MathTest {
            problems,
            difficulty,
            start_time,
            end_time: None,
        }
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

    fn format_test_duration(&self, seconds: &u64) -> String {
        const SECS_IN_HOUR: u64 = 3600;
        const SECS_IN_MIN: u64 = 60;

        fn format_seconds(secs: &u64) -> String {
            if secs.lt(&10) {
                format!("0{}", secs)
            } else {
                format!("{}", secs)
            }
        }

        fn format_mins(mins: &u64) -> String {
            if mins.lt(&10) {
                format!("0{}", mins)
            } else {
                format!("{}", mins)
            }
        }

        fn format_hrs(hrs: &u64) -> String {
            if hrs.lt(&10) {
                format!("0{}", hrs)
            } else {
                format!("{}", hrs)
            }
        }

        if seconds.ge(&SECS_IN_HOUR) {
            let hrs = seconds / 60 / 60;
            let mins = seconds / 60 % 60;
            let secs = mins % 60;
            format!(
                "{}:{}:{}",
                format_hrs(&hrs),
                format_mins(&mins),
                format_seconds(&secs)
            )
        } else if seconds.ge(&SECS_IN_MIN) {
            let mins = seconds / 60;
            let secs = seconds % 60;
            format!("{}:{}", format_mins(&mins), format_seconds(&secs))
        } else {
            format!("{} seconds", format_seconds(seconds))
        }
    }

    fn add_score_to_results(
        &self,
        mut results: String,
        total_questions: &usize,
        total_incorrect: &usize,
        total_duration_secs: &u64,
    ) -> String {
        results.push_str(
            format!(
                "\nDifficulty: Level {}\nTotal Questions: {}\nTotal Incorrect: {}\nScore: {}\nTime: {}",
                self.difficulty,
                total_questions,
                total_incorrect,
                100 - (100 / total_questions * total_incorrect),
                self.format_test_duration(total_duration_secs)
            )
            .as_str(),
        );

        results
    }

    pub fn get_results(&mut self) -> String {
        let total_questions = self.problems.len();
        let mut total_incorrect = 0;
        let mut results = String::from("###################### RESULTS ######################\n");
        self.end_time = Some(Instant::now());

        for i in 0..total_questions {
            let problem = &self.problems[i];

            if let Some(outcome) = problem.user_correct {
                if outcome {
                    results = match problem.operation {
                        Operation::Addition(ref operator) => self.add_correct_answer_to_results(
                            results,
                            &problem.first_number,
                            operator,
                            &problem.second_number,
                            &problem.expected_answer,
                        ),
                        Operation::Subtraction(ref operator) => self.add_correct_answer_to_results(
                            results,
                            &problem.first_number,
                            operator,
                            &problem.second_number,
                            &problem.expected_answer,
                        ),
                        Operation::Multiplication(ref operator) => self
                            .add_correct_answer_to_results(
                                results,
                                &problem.first_number,
                                operator,
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
                                    operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                            Operation::Subtraction(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                            Operation::Multiplication(ref operator) => self
                                .add_incorrect_answer_to_results(
                                    results,
                                    &problem.first_number,
                                    operator,
                                    &problem.second_number,
                                    &problem.expected_answer,
                                    &user_answer,
                                ),
                        }
                    }
                }
            }
        }

        let mut duration_secs = 0;
        if let Some(end_time) = self.end_time {
            duration_secs = end_time.duration_since(self.start_time).as_secs();
        }

        results =
            self.add_score_to_results(results, &total_questions, &total_incorrect, &duration_secs);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_gets_updated_with_correct_answer() {
        let mock_first_number = 1;
        let mock_second_number = 1;
        let mock_operator = '+';
        let mock_expected_answer = mock_first_number + mock_second_number;
        let mock_results = String::new();
        let mock_math_test = MathTest::new(1);

        let results = mock_math_test.add_correct_answer_to_results(
            mock_results,
            &mock_first_number,
            &mock_operator,
            &mock_second_number,
            &mock_expected_answer,
        );
        let expected = String::from("\nCORRECT 1 + 1 = 2\n");

        assert_eq!(expected, results)
    }

    #[test]
    fn test_results_gets_updated_with_incorrect_answer() {
        let mock_first_number = 1;
        let mock_second_number = 1;
        let mock_operator = '+';
        let mock_user_answer = 1;
        let mock_expected_answer = mock_first_number + mock_second_number;
        let mock_results = String::new();
        let mock_math_test = MathTest::new(1);

        let results = mock_math_test.add_incorrect_answer_to_results(
            mock_results,
            &mock_first_number,
            &mock_operator,
            &mock_second_number,
            &mock_expected_answer,
            &mock_user_answer,
        );
        let expected = String::from("\nINCORRECT\nUser Answer: 1 + 1 = 1\nExpected: 1 + 1 = 2\n");

        assert_eq!(expected, results)
    }

    #[test]
    fn test_results_gets_updated_with_score() {
        let mock_total_questions = 10;
        let mock_total_incorrect = 5;
        let mock_difficulty = 1;
        let mock_results = String::new();
        let mock_math_test = MathTest::new(mock_difficulty);
        let mock_duration = &3733;

        let results = mock_math_test.add_score_to_results(
            mock_results,
            &mock_total_questions,
            &mock_total_incorrect,
            mock_duration,
        );
        let expected = String::from(
            "\nDifficulty: Level 1\nTotal Questions: 10\nTotal Incorrect: 5\nScore: 50\nTime: 01:02:02",
        );

        assert_eq!(expected, results)
    }

    #[test]
    fn test_math_test_produces_results() {
        let mock_correct_addition_problem = MathProblem {
            first_number: 1,
            second_number: 1,
            operation: Operation::Addition('+'),
            user_answer: Some(2),
            expected_answer: 2,
            user_correct: Some(true),
            ui_string: String::from("1 + 1 = ?"),
        };
        let mock_incorrect_addition_problem = MathProblem {
            first_number: 1,
            second_number: 1,
            operation: Operation::Addition('+'),
            user_answer: Some(1),
            expected_answer: 2,
            user_correct: Some(false),
            ui_string: String::from("1 + 1 = ?"),
        };
        let mock_correct_subtraction_problem = MathProblem {
            first_number: 2,
            second_number: 1,
            operation: Operation::Subtraction('-'),
            user_answer: Some(1),
            expected_answer: 1,
            user_correct: Some(true),
            ui_string: String::from("2 - 1 = ?"),
        };
        let mock_incorrect_subtraction_problem = MathProblem {
            first_number: 2,
            second_number: 1,
            operation: Operation::Addition('-'),
            user_answer: Some(2),
            expected_answer: 1,
            user_correct: Some(false),
            ui_string: String::from("2 - 1 = ?"),
        };
        let mock_correct_multiplication_problem = MathProblem {
            first_number: 2,
            second_number: 2,
            operation: Operation::Multiplication('*'),
            user_answer: Some(4),
            expected_answer: 4,
            user_correct: Some(true),
            ui_string: String::from("2 * 2 = ?"),
        };
        let mock_incorrect_multiplication_problem = MathProblem {
            first_number: 2,
            second_number: 2,
            operation: Operation::Multiplication('*'),
            user_answer: Some(2),
            expected_answer: 4,
            user_correct: Some(false),
            ui_string: String::from("2 * 2 = ?"),
        };
        let mut mock_math_test = MathTest::new(1);
        mock_math_test.problems = vec![
            mock_correct_addition_problem,
            mock_incorrect_addition_problem,
            mock_correct_subtraction_problem,
            mock_incorrect_subtraction_problem,
            mock_correct_multiplication_problem,
            mock_incorrect_multiplication_problem,
        ];
        let results = mock_math_test.get_results();
        let expected = String::from("###################### RESULTS ######################\n\nCORRECT 1 + 1 = 2\n\nINCORRECT\nUser Answer: 1 + 1 = 1\nExpected: 1 + 1 = 2\n\nCORRECT 2 - 1 = 1\n\nINCORRECT\nUser Answer: 2 - 1 = 2\nExpected: 2 - 1 = 1\n\nCORRECT 2 * 2 = 4\n\nINCORRECT\nUser Answer: 2 * 2 = 2\nExpected: 2 * 2 = 4\n\nDifficulty: Level 1\nTotal Questions: 6\nTotal Incorrect: 3\nScore: 52\nTime: 00 seconds");

        assert_eq!(expected, results)
    }
}

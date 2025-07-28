
use crate::app_state::AppState;
use crate::question::Question;
use crate::testconfig::TestConfig;
use std::time::Instant;

#[derive(Default)]
pub struct App {
    pub config: TestConfig,
    pub state: AppState,
    pub selected_config_item: usize,
    pub questions_answered: usize,
    pub current_question: Option<Question>,
    pub user_answer: String,
    pub start_time: Option<Instant>,
    pub editing_config: bool,
    pub config_input: String,
    pub config_input_started: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_test(&mut self) {
        self.questions_answered = 0;
        self.user_answer.clear();
        self.current_question = Some(Question::new(&self.config));
        self.state = AppState::Testing;
        self.start_time = Some(Instant::now());
    }

    pub fn submit_answer(&mut self) {
        if let Some(question) = &self.current_question {
            if let Ok(answer) = self.user_answer.parse::<i32>() {
                if answer == question.correct_answer {
                    self.questions_answered += 1;
                    self.user_answer.clear();

                    if self.is_time_up() {
                        self.state = AppState::Results;
                    } else {
                        self.current_question = Some(Question::new(&self.config));
                    }
                }
                 If answer is wrong, keep the input and don't advance
            }
        }
    }

    pub fn time_remaining(&self) -> u64 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs();
            if elapsed >= self.config.time_limit {
                0
            } else {
                self.config.time_limit - elapsed
            }
        } else {
            self.config.time_limit
        }
    }

    pub fn is_time_up(&self) -> bool {
        self.time_remaining() == 0
    }

    pub fn get_config_value(&self, index: usize) -> i32 {
        match index {
            0 => self.config.add_min_a,
            1 => self.config.add_max_a,
            2 => self.config.add_min_b,
            3 => self.config.add_max_b,
            4 => self.config.mul_min_a,
            5 => self.config.mul_max_a,
            6 => self.config.mul_min_b,
            7 => self.config.mul_max_b,
            8 => self.config.time_limit as i32,
            _ => 0,
        }
    }

    pub fn set_config_value(&mut self, index: usize, value: i32) {
        match index {
            0 => self.config.add_min_a = value,
            1 => self.config.add_max_a = value,
            2 => self.config.add_min_b = value,
            3 => self.config.add_max_b = value,
            4 => self.config.mul_min_a = value,
            5 => self.config.mul_max_a = value,
            6 => self.config.mul_min_b = value,
            7 => self.config.mul_max_b = value,
            8 => self.config.time_limit = value as u64,
            _ => {}
        }
    }

    pub fn get_config_label(&self, index: usize) -> &str {
        match index {
            0 => "Addition Min A",
            1 => "Addition Max A",
            2 => "Addition Min B",
            3 => "Addition Max B",
            4 => "Multiplication Min A",
            5 => "Multiplication Max A",
            6 => "Multiplication Min B",
            7 => "Multiplication Max B",
            8 => "Time Limit (seconds)",
            9 => ">>> START TEST <<<",
            _ => "Unknown",
        }
    }

    pub fn start_editing(&mut self) {
        self.editing_config = true;
        self.config_input = self.get_config_value(self.selected_config_item).to_string();
        self.config_input_started = false;
    }

    pub fn finish_editing(&mut self) {
        if let Ok(value) = self.config_input.parse::<i32>() {
            if value > 0 {
                self.set_config_value(self.selected_config_item, value);
            }
        }
        self.editing_config = false;
        self.config_input.clear();
        self.config_input_started = false;
    }

    pub fn cancel_editing(&mut self) {
        self.editing_config = false;
        self.config_input.clear();
        self.config_input_started = false;
    }
}

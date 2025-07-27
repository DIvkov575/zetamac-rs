use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use rand::Rng;
use std::{
    io,
    time::{Duration, Instant},
};

#[derive(Clone)]
struct TestConfig {
    add_min_a: i32,
    add_max_a: i32,
    add_min_b: i32,
    add_max_b: i32,
    mul_min_a: i32,
    mul_max_a: i32,
    mul_min_b: i32,
    mul_max_b: i32,
    time_limit: u64,
}

#[derive(Clone, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
struct Question {
    a: i32,
    b: i32,
    operation: Operation,
    correct_answer: i32,
}

struct App {
    config: TestConfig,
    state: AppState,
    selected_config_item: usize,
    questions_answered: usize,
    current_question: Option<Question>,
    user_answer: String,
    start_time: Option<Instant>,
    editing_config: bool,
    config_input: String,
    config_input_started: bool,
}

#[derive(PartialEq)]
enum AppState {
    Configuration,
    Testing,
    Results,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            add_min_a: 2,
            add_max_a: 100,
            add_min_b: 2,
            add_max_b: 100,
            mul_min_a: 2,
            mul_max_a: 12,
            mul_min_b: 2,
            mul_max_b: 100,
            time_limit: 120,
        }
    }
}

impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/",
        }
    }

    fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            _ => Operation::Division,
        }
    }
}

impl Question {
    fn new(config: &TestConfig) -> Self {
        let mut rng = rand::rng();
        let operation = Operation::random();
        
        let (a, b, correct_answer) = match operation {
            Operation::Addition => {
                let a = rng.random_range(config.add_min_a..=config.add_max_a);
                let b = rng.random_range(config.add_min_b..=config.add_max_b);
                (a, b, a + b)
            }
            Operation::Subtraction => {
                // Use addition ranges inversely: generate result + b = a
                let result = rng.random_range(config.add_min_a..=config.add_max_a);
                let b = rng.random_range(config.add_min_b..=config.add_max_b);
                let a = result + b;
                (a, b, result)
            }
            Operation::Multiplication => {
                let a = rng.random_range(config.mul_min_a..=config.mul_max_a);
                let b = rng.random_range(config.mul_min_b..=config.mul_max_b);
                (a, b, a * b)
            }
            Operation::Division => {
                // Use multiplication ranges inversely: generate result * b = a
                let result = rng.random_range(config.mul_min_a..=config.mul_max_a);
                let b = rng.random_range(std::cmp::max(config.mul_min_b, 1)..=config.mul_max_b);
                let a = result * b;
                (a, b, result)
            }
        };

        Self {
            a,
            b,
            operation,
            correct_answer,
        }
    }

    fn display(&self) -> String {
        format!("{} {} {} = ?", self.a, self.operation.symbol(), self.b)
    }
}

impl App {
    fn new() -> Self {
        Self {
            config: TestConfig::default(),
            state: AppState::Configuration,
            selected_config_item: 0,
            questions_answered: 0,
            current_question: None,
            user_answer: String::new(),
            start_time: None,
            editing_config: false,
            config_input: String::new(),
            config_input_started: false,
        }
    }

    fn start_test(&mut self) {
        self.questions_answered = 0;
        self.user_answer.clear();
        self.current_question = Some(Question::new(&self.config));
        self.state = AppState::Testing;
        self.start_time = Some(Instant::now());
    }

    fn submit_answer(&mut self) {
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
                // If answer is wrong, keep the input and don't advance
            }
        }
    }

    fn time_remaining(&self) -> u64 {
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

    fn is_time_up(&self) -> bool {
        self.time_remaining() == 0
    }

    fn get_config_value(&self, index: usize) -> i32 {
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

    fn set_config_value(&mut self, index: usize, value: i32) {
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

    fn get_config_label(&self, index: usize) -> &str {
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

    fn start_editing(&mut self) {
        self.editing_config = true;
        self.config_input = self.get_config_value(self.selected_config_item).to_string();
        self.config_input_started = false;
    }

    fn finish_editing(&mut self) {
        if let Ok(value) = self.config_input.parse::<i32>() {
            if value > 0 {
                self.set_config_value(self.selected_config_item, value);
            }
        }
        self.editing_config = false;
        self.config_input.clear();
        self.config_input_started = false;
    }

    fn cancel_editing(&mut self) {
        self.editing_config = false;
        self.config_input.clear();
        self.config_input_started = false;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if app.state == AppState::Testing && app.is_time_up() {
            app.state = AppState::Results;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::Configuration => {
                        if app.editing_config {
                            match key.code {
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    if !app.config_input_started {
                                        app.config_input.clear();
                                        app.config_input_started = true;
                                    }
                                    app.config_input.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.config_input.pop();
                                }
                                KeyCode::Enter => {
                                    app.finish_editing();
                                }
                                KeyCode::Esc => {
                                    app.cancel_editing();
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Up => {
                                    if app.selected_config_item > 0 {
                                        app.selected_config_item -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.selected_config_item < 9 {
                                        app.selected_config_item += 1;
                                    }
                                }
                                KeyCode::Enter => {
                                    if app.selected_config_item == 9 {
                                        app.start_test();
                                    } else if app.selected_config_item == 8 {
                                        app.start_editing();
                                    } else {
                                        app.start_editing();
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                    AppState::Testing => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
                            app.user_answer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.user_answer.pop();
                        }
                        KeyCode::Enter => {
                            if !app.user_answer.is_empty() {
                                app.submit_answer();
                            }
                        }
                        KeyCode::Esc => {
                            app.state = AppState::Configuration;
                            app.start_time = None;
                        }
                        _ => {}
                    },
                    AppState::Results => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') => {
                            app.state = AppState::Configuration;
                            app.start_time = None;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    match app.state {
        AppState::Configuration => draw_configuration(f, app),
        AppState::Testing => draw_testing(f, app),
        AppState::Results => draw_results(f, app),
    }
}

fn draw_configuration(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(f.area());

    let title = Paragraph::new("ZETAMEC - Configuration")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let mut config_items = Vec::new();
    for i in 0..10 {
        let label = app.get_config_label(i);
        
        if i == 9 {
            // Start button - no value, just the label
            config_items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    label,
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ])));
        } else {
            let value = if i == 8 {
                app.config.time_limit.to_string()
            } else {
                app.get_config_value(i).to_string()
            };
            
            let display_value = if app.editing_config && app.selected_config_item == i {
                &app.config_input
            } else {
                &value
            };

            config_items.push(ListItem::new(Line::from(vec![
                Span::raw(format!("{}: ", label)),
                Span::styled(
                    display_value.clone(),
                    Style::default().fg(Color::Yellow),
                ),
            ])));
        }
    }

    let config_list = List::new(config_items)
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");
    f.render_stateful_widget(config_list, chunks[1], &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_config_item)));

    let instructions = if app.editing_config {
        "Type number, Enter: Save, Esc: Cancel"
    } else {
        "↑/↓: Navigate  Enter: Edit/Start  Q: Quit"
    };
    
    let instructions_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions_widget, chunks[2]);
}

fn draw_testing(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("ZETAMEC - In Progress")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let time_remaining = format!("Time: {}s", app.time_remaining());
    let score = format!("Questions Answered: {}", app.questions_answered);
    
    let info = Paragraph::new(format!("{} | {}", time_remaining, score))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);

    if let Some(question) = &app.current_question {
        let question_text = Paragraph::new(question.display())
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Question"));
        f.render_widget(question_text, chunks[2]);

        let answer_input = Paragraph::new(app.user_answer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Your Answer"));
        f.render_widget(answer_input, chunks[3]);
    }

    let time_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Time Remaining"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(app.time_remaining() as f64 / app.config.time_limit as f64);
    f.render_widget(time_gauge, chunks[4]);
}

fn draw_results(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("ZETAMEC - Results")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let results_text = format!(
        "Final Score: {} questions answered correctly\n\nTest completed in {} seconds",
        app.questions_answered,
        app.config.time_limit
    );

    let results = Paragraph::new(results_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Final Results"));
    f.render_widget(results, chunks[1]);

    let instructions = Paragraph::new("R: Return to Configuration  Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}
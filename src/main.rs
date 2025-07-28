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
use crate::app::App;
use crate::app_state::AppState;
use std::io;
use std::time::Duration;

mod app;
mod app_state;
mod operation;
mod question;
mod testconfig;







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

    let title = Paragraph::new("ZETAMAC - Configuration")
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

    let title = Paragraph::new("ZETAMAC - In Progress")
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
        .ratio(app.time_remaining() as f64 / app.config.time_limit as f64)
        .label(format!("{}s", app.time_remaining()));
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

    let title = Paragraph::new("ZETAMAC - Results")
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
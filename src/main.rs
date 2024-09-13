use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::process::Command;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... main function content ...
}

enum InputMode {
    Normal,
    Editing,
}

struct App {
    input: String,
    logs: Vec<String>,
    input_mode: InputMode,
    filter: String,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            logs: Vec::new(),
            input_mode: InputMode::Normal,
            filter: String::new(),
        }
    }

    fn fetch_logs(&mut self) {
        let output = Command::new("journalctl")
            .arg("-n")
            .arg("1000")
            .output()
            .expect("Failed to execute journalctl");

        self.logs = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
    }

    fn filter_logs(&self) -> Vec<String> {
        self.logs
            .iter()
            .filter(|log| log.to_lowercase().contains(&self.filter.to_lowercase()))
            .cloned()
            .collect()
    }

    fn save_logs(&self) -> io::Result<()> {
        let filtered_logs = self.filter_logs();
        std::fs::write("filtered_logs.txt", filtered_logs.join("\n"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.fetch_logs();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let (msg, style) = match app.input_mode {
                InputMode::Normal => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing, "),
                        Span::styled("Ctrl+S", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to save logs."),
                    ],
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                InputMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to apply filter"),
                    ],
                    Style::default(),
                ),
            };
            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let input = Paragraph::new(app.input.as_ref())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Filter"));
            f.render_widget(input, chunks[1]);

            let logs: Vec<ListItem> = app
                .filter_logs()
                .iter()
                .map(|log| ListItem::new(log.as_str()))
                .collect();
            let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("Logs"));
            f.render_widget(logs, chunks[2]);

            if let InputMode::Editing = app.input_mode {
                f.set_cursor(chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1)
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                        app.save_logs()?;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.filter = app.input.drain(..).collect();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}


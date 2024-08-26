use std::error;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style, Stylize},
    symbols::scrollbar,
    text::{Line, Masked, Span, Text},
    widgets::{
        Block, BorderType, Borders, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
    },
    Frame,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum AppState {
    Normal,
    Editing,
    Quit,
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    /// counter
    pub state: AppState,
    pub input: tui_input::Input,
    pub debug: String,
    pub messages: Vec<String>,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Normal,
            input: tui_input::Input::default(),
            messages: vec![],
            debug: String::new(),
            vertical_scroll: 0,
            horizontal_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            horizontal_scroll_state: ScrollbarState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(2);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        self.debug += &format!("scroll down {}", self.vertical_scroll).to_string();
    }

    pub fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(2);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        self.debug += &format!("scroll up {}", self.vertical_scroll).to_string();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }
    pub fn editing(&mut self) {
        self.state = AppState::Editing;
    }
    pub fn normal(&mut self) {
        self.state = AppState::Normal;
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Min(3),
                    Constraint::Percentage(90),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.area());

        let msg_help = vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("i ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to start editing."),
        ];
        // let ifdfd = vec![
        //     Span::raw("Press "),
        //     Span::styled("i ", Style::default().add_modifier(Modifier::BOLD)),
        //     Span::raw("to write, "),
        // ];
        // let style = Style::default().add_modifier(Modifier::RAPID_BLINK);
        // let text = Text::from(Line::from(msg_help)).style(style);
        // let help_message = Paragraph::new(text);
        // frame.render_widget(help_message, chunks[0]);

        let debug = Paragraph::new(self.debug.as_str());
        frame.render_widget(debug, chunks[2]);
        self.debug.clear();
        // Words made "loooong" to demonstrate line breaking.
        let s =
            "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
        let mut long_line = s.repeat(usize::from(frame.area().width) / s.len() + 4);
        long_line.push('\n');

        // let chunks = Layout::vertical([
        //     Constraint::Min(1),
        //     Constraint::Percentage(25),
        //     Constraint::Percentage(25),
        //     Constraint::Percentage(25),
        //     Constraint::Percentage(25),
        // ])
        // .split(area);

        let text = vec![
            Line::from("This is a line "),
            Line::from("This is a line   ".red()),
            Line::from("This is a line".on_dark_gray()),
            Line::from("This is a longer line".crossed_out()),
            Line::from(long_line.clone()),
            Line::from("This is a line".reset()),
            Line::from(vec![
                Span::raw("Masked text: "),
                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
            ]),
            Line::from("This is a line "),
            Line::from("This is a line   ".red()),
            Line::from("This is a line".on_dark_gray()),
            Line::from("This is a longer line".crossed_out()),
            Line::from(long_line.clone()),
            Line::from("This is a line".reset()),
            Line::from(vec![
                Span::raw("Masked text: "),
                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
            ]),
        ];
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(text.len());
        // self.horizontal_scroll_state = self.horizontal_scroll_state.content_length(long_line.len());

        let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());

        let title = Block::new()
            .title_alignment(Alignment::Center)
            .title("Use h j k l or ◄ ▲ ▼ ► to scroll ".bold());
        frame.render_widget(title, chunks[1]);

        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(create_block("Messages"))
            .scroll((self.vertical_scroll as u16, 0));
        frame.render_widget(paragraph, chunks[1]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[1],
            &mut self.vertical_scroll_state,
        );

        let width = chunks[0].width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(Style::default())
            .scroll((0, scroll as u16))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title_bottom(msg_help)
                    .title_alignment(Alignment::Right),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(input, chunks[0]);
    }
}

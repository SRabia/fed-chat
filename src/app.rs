use crate::msg::MsgView;
use std::error;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
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
    pub msgview: MsgView,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Normal,
            input: tui_input::Input::default(),
            msgview: MsgView::default(),
            debug: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scroll_down(&mut self) {
        self.msgview.down();
    }

    pub fn top(&mut self) {
        self.msgview.top();
    }

    pub fn bottom(&mut self) {
        self.msgview.bottom();
    }

    pub fn scroll_up(&mut self) {
        self.msgview.up();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.msgview.update_max_scroll_offset();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }
    pub fn accept_message_input(&mut self) {
        if !self.input.value().is_empty() {
            self.msgview.add_msg(self.input.value().into());
            self.input.reset();
            self.msgview.down_one_msg();
        }
    }

    pub fn editing(&mut self) {
        self.state = AppState::Editing;
        self.input.reset();
    }
    pub fn normal(&mut self) {
        self.state = AppState::Normal;
    }

    fn draw_in_normal_state(
        &self,
        area_msgview: Rect,
        area_input: Rect,
        area_help: Rect,
        frame: &mut Frame,
    ) {
        let msg_help = vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
        ];

        let msg_help = self.get_help_msg_style(msg_help);
        frame.render_widget(
            Paragraph::new(msg_help).alignment(Alignment::Right),
            area_help,
        );

        let input_text = "press i to start writing";
        let input = Paragraph::new(input_text.fg(Color::DarkGray))
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title_alignment(Alignment::Right),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(input, area_input);
        //improve this should not clone
        frame.render_widget(self.msgview.clone(), area_msgview);
    }

    fn get_help_msg_style<'a>(&self, spans: Vec<Span<'a>>) -> Text<'a> {
        Text::from(Line::from(spans)).style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::RAPID_BLINK),
        )
    }

    fn draw_in_editing_state(
        &self,
        area_msgview: Rect,
        area_input: Rect,
        area_help: Rect,
        frame: &mut Frame,
    ) {
        let msg_help = vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit editing, "),
            Span::styled("Enter ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to send message."),
        ];
        let msg_help = self.get_help_msg_style(msg_help);

        frame.render_widget(
            Paragraph::new(msg_help).alignment(Alignment::Right),
            area_help,
        );

        let width = area_input.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(Style::default().fg(Color::LightBlue))
            .scroll((0, scroll as u16))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title_alignment(Alignment::Right),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(input, area_input);
        let offset_cursor = (self.input.visual_cursor().max(scroll) - scroll) as u16 + 1;
        let cursor_position = Position::new(area_input.x + offset_cursor, area_input.y + 1);
        frame.set_cursor_position(cursor_position);

        //TODO: refactor to remove clone

        frame.render_widget(self.msgview.clone(), area_msgview);
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(90), // msgview
                    Constraint::Min(3),         // input
                    Constraint::Length(1),      //help
                    Constraint::Min(0),         //debug
                ]
                .as_ref(),
            )
            .split(frame.area());

        if !self.debug.is_empty() {
            let debug = Paragraph::new(self.debug.as_str());
            frame.render_widget(debug, main_layout[2]);
            self.debug.clear();
        }
        match self.state {
            AppState::Normal => {
                self.draw_in_normal_state(main_layout[0], main_layout[1], main_layout[3], frame)
            }

            AppState::Editing => {
                self.draw_in_editing_state(main_layout[0], main_layout[1], main_layout[3], frame)
            }
            _ => {}
        }
    }
}

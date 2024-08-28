// use crate::msg::MsgView;
use std::error;

use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint::{self, Length, Percentage},
        Direction, Layout, Position, Rect,
    },
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text, ToSpan},
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Widget, Wrap,
    },
    Frame,
};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const HEIGHT_SPACER: u16 = 0;

// when scrolling to bottom will show {SCROLL_BOTTOM_NB_MSG_IN_SCOPE} nb of messages
// const SCROLL_BOTTOM_NB_MSG_IN_SCOPE: usize = 2;
const DEFAULT_CHAT_BOX_HEIGHT: u16 = 4;
// const HEIGHT_MSG_VIEW: u16 = DEFAULT_CHAT_BOX_HEIGHT + HEIGHT_SPACER;

const SCROLL_BOTTOM_NB_CHAT_BOX_VISIBLE: i32 = 2;
#[derive(Debug, PartialEq)]
pub enum AppState {
    Normal,
    Editing,
    Quit,
}
/// Application.
#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub chat_input: tui_input::Input,
    pub debug: String, //TODO: remove this
    chats: Vec<String>,
    scroll_offset_chats: u16,
    max_scroll_offset_chats: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Normal,
            chat_input: tui_input::Input::default(),
            chats: Vec::new(),
            debug: String::new(),
            max_scroll_offset_chats: 0,
            scroll_offset_chats: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scroll_down_chat(&mut self) {
        self.scroll_offset_chats = self
            .scroll_offset_chats
            .saturating_add(1)
            .min(self.max_scroll_offset_chats);
    }

    pub fn scroll_top_chat(&mut self) {
        self.scroll_offset_chats = 0;
    }

    pub fn scroll_botton_chat(&mut self) {
        self.scroll_offset_chats = self.max_scroll_offset_chats;
    }

    pub fn scroll_up_chat(&mut self) {
        self.scroll_offset_chats = self.scroll_offset_chats.saturating_sub(1);
    }

    pub fn tick(&mut self) {}

    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    pub fn accept_chat_input(&mut self) {
        if !self.chat_input.value().is_empty() {
            self.chats.push(self.chat_input.value().into());
            self.chat_input.reset();
            self.scroll_botton_chat();
        }
    }

    pub fn editing(&mut self) {
        self.state = AppState::Editing;
        self.chat_input.reset();
    }
    pub fn normal(&mut self) {
        self.state = AppState::Normal;
    }

    fn draw_in_normal_state(
        &mut self,
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

        let msg_help = self.get_help_display_text_style(msg_help);
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
        self.draw_chat_discussion(area_msgview, frame);
    }

    fn get_help_display_text_style<'a>(&self, spans: Vec<Span<'a>>) -> Text<'a> {
        Text::from(Line::from(spans)).style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::RAPID_BLINK),
        )
    }

    fn calculate_chat_box_rect_height(msg: &str, area: Rect) -> u16 {
        let wid_text_perc = msg.len() as f32 / area.width as f32;

        if wid_text_perc < 0.75 {
            DEFAULT_CHAT_BOX_HEIGHT
        } else {
            ((wid_text_perc) / 0.75) as u16 + DEFAULT_CHAT_BOX_HEIGHT
        }
    }

    fn draw_single_chat(&self, msg: &str, area: Rect, buf: &mut Buffer) {
        let author = "toto le fromage"
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD);

        let msg = msg.to_span();
        let max_len = msg.width().max(author.width());
        let wid_text_perc = max_len as f32 / area.width as f32;
        let chat_box_wid = if wid_text_perc < 0.75 {
            max_len as u16 + 2
        } else {
            // let height = ((wid_text_perc - 0.75) / 0.75) as u16 + DEFAULT_CHAT_BOX_HEIGHT;
            (area.width as f32 * 0.75) as u16
        };

        // let c: &[Constraint] = &[Min(10), Percentage(constrains_text_box)];
        let c: &[Constraint] = &[Percentage(100), Length(chat_box_wid)];
        let [area, _] = Layout::vertical([Length(area.height), Length(HEIGHT_SPACER)]).areas(area);
        let area_chat = Layout::horizontal(c).split(area);
        // let color = LENGTH_COLOR;
        // let fg = Color::White;

        let debug = format!("len {}:{}", wid_text_perc, chat_box_wid)
            .fg(Color::LightRed)
            .into_right_aligned_line();

        // let title = "me".to_string();
        let content = msg.to_string();
        // let content = format!("{}", self.msg.as_str());
        // let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .title(author)
            .padding(Padding::new(0, 0, 1, 0))
            .title_bottom(debug)
            // .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_type(ratatui::widgets::BorderType::Rounded);
        // .border_style(Style::reset().fg(color).reversed())
        // .style(Style::default().fg(fg).bg(color));
        let content = Paragraph::new(content)
            .right_aligned()
            .block(block)
            .wrap(Wrap { trim: true });

        content.render(area_chat[1], buf); // right align
    }

    fn draw_chat_discussion(&mut self, area: Rect, frame: &mut Frame) {
        //TODO: this is going to blow up when shit tone of messages have to be rendered!
        let mut total_height_chat_boxs = 0;
        let height = self.max_scroll_offset_chats;
        let msg_area = Rect::new(0, 0, area.width, height + area.height);
        let mut msg_buf = Buffer::empty(msg_area);

        let scrollbar_needed = self.scroll_offset_chats != 0 || height > area.height;
        let content_area = if scrollbar_needed {
            Rect {
                width: msg_area.width - 1,
                ..msg_area
            }
        } else {
            msg_area
        };

        let scroll_count_stop =
            (self.chats.len() as i32 - SCROLL_BOTTOM_NB_CHAT_BOX_VISIBLE).max(0) as usize;

        let constraints_vertical: Vec<Constraint> = self
            .chats
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let height = Self::calculate_chat_box_rect_height(m, content_area);
                if i < scroll_count_stop {
                    total_height_chat_boxs += height;
                }
                Length(height)
            })
            .collect();

        let msg_grid = Layout::vertical(constraints_vertical.as_slice()).split(content_area);
        for (i, m) in self.chats.iter().enumerate() {
            self.draw_single_chat(m.as_str(), msg_grid[i], &mut msg_buf);
        }
        //TODO: max a small offset to at least show some message at bottom
        self.max_scroll_offset_chats = total_height_chat_boxs;

        self.debug += &format!("got height{}", self.max_scroll_offset_chats);

        let visible_content = msg_buf
            .content
            .into_iter()
            .skip((msg_area.width * self.scroll_offset_chats) as usize)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            frame.buffer_mut()[(area.x + x, area.y + y)] = cell;
        }

        if scrollbar_needed {
            let mut state = ScrollbarState::new(self.max_scroll_offset_chats as usize)
                .position(self.scroll_offset_chats as usize);
            let s = Scrollbar::new(ScrollbarOrientation::VerticalRight); //.render(area, buf, &mut state);
            frame.render_stateful_widget(s, area, &mut state);
        }
    }

    fn draw_in_editing_state(
        &mut self,
        area_msgview: Rect,
        area_input: Rect,
        area_help: Rect,
        frame: &mut Frame,
    ) {
        let msg_help = vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit editing, "),
            Span::styled(
                "<Ctrl-Enter> ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to send message."),
        ];
        let msg_help = self.get_help_display_text_style(msg_help);

        frame.render_widget(
            Paragraph::new(msg_help).alignment(Alignment::Right),
            area_help,
        );

        let width = area_input.width.max(3) - 3;
        let scroll = self.chat_input.visual_scroll(width as usize);
        let input = Paragraph::new(self.chat_input.value())
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
        let offset_cursor = (self.chat_input.visual_cursor().max(scroll) - scroll) as u16 + 1;
        let cursor_position = Position::new(area_input.x + offset_cursor, area_input.y + 1);
        frame.set_cursor_position(cursor_position);

        self.draw_chat_discussion(area_msgview, frame);
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

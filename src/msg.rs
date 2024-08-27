use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint::{self, Fill, Length, Max, Min, Percentage, Ratio},
        Layout, Rect,
    },
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget, Wrap,
    },
};

const SPACER_HEIGHT: u16 = 0;
const ILLUSTRATION_HEIGHT: u16 = 4;
const EXAMPLE_HEIGHT: u16 = ILLUSTRATION_HEIGHT + SPACER_HEIGHT;

// priority 2
const MIN_COLOR: Color = tailwind::BLUE.c900;
const MAX_COLOR: Color = tailwind::BLUE.c800;
// priority 3
const LENGTH_COLOR: Color = tailwind::SLATE.c700;
const PERCENTAGE_COLOR: Color = tailwind::SLATE.c800;
const RATIO_COLOR: Color = tailwind::SLATE.c900;
// priority 4
const FILL_COLOR: Color = tailwind::SLATE.c950;

#[derive(Default, Clone, Debug)]
pub struct MsgList {
    messages: Vec<String>,
    scroll_offset: u16,
    max_scroll_offset: u16,
}

/// Tabs for the different examples
///
/// The order of the variants is the order in which they are displayed.

impl MsgList {
    pub fn update_max_scroll_offset(&mut self) {
        if !self.messages.is_empty() {
            self.max_scroll_offset = (self.messages.len() as u16 - 1) * EXAMPLE_HEIGHT;
        }
    }

    pub fn up(&mut self) {
        // self.messages.push(format!("up {}", self.scroll_offset));
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
    pub fn add_msg(&mut self, text: String) {
        self.messages.push(text);
    }

    pub fn down(&mut self) {
        // self.messages.push(format!("down {}", self.scroll_offset));
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(1)
            .min(self.max_scroll_offset);
    }

    pub fn top(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset;
    }
}

impl Widget for MsgList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height = self.messages.len() as u16 * EXAMPLE_HEIGHT;
        let msg_area = Rect::new(0, 0, area.width, height + area.height);
        let mut msg_buf = Buffer::empty(msg_area);

        let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
        let content_area = if scrollbar_needed {
            Rect {
                width: msg_area.width - 1,
                ..msg_area
            }
        } else {
            msg_area
        };

        let constraints: Vec<Constraint> = self
            .messages
            .iter()
            .map(|_| Max(EXAMPLE_HEIGHT)) //here have something more dynamic in height
            .collect();

        // self.selected_tab.render(content_area, &mut demo_buf);
        let msg_grid = Layout::vertical(constraints.as_slice()).split(content_area);
        for (i, m) in self.messages.iter().enumerate() {
            Msg::new(m, Alignment::Right).render(msg_grid[i], &mut msg_buf);
        }

        let visible_content = msg_buf
            .content
            .into_iter()
            .skip((msg_area.width * self.scroll_offset) as usize)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            buf[(area.x + x, area.y + y)] = cell;
        }

        if scrollbar_needed {
            let mut state = ScrollbarState::new(self.max_scroll_offset as usize)
                .position(self.scroll_offset as usize);
            Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
        }
    }
}

impl MsgList {
    // fn render_axis(area: Rect, buf: &mut Buffer) {
    //     let width = area.width as usize;
    //     // a bar like `<----- 80 px ----->`
    //     let width_label = format!("{width} px");
    //     let width_bar = format!(
    //         "<{width_label:-^width$}>",
    //         width = width - width_label.len() / 2
    //     );
    //     Paragraph::new(width_bar.dark_gray())
    //         .centered()
    //         .block(Block::new().padding(Padding {
    //             left: 0,
    //             right: 0,
    //             top: 1,
    //             bottom: 0,
    //         }))
    //         .render(area, buf);
    // }
}

struct Msg {
    msg: String,
    align: Alignment,
}

impl Msg {
    fn new(msg: &str, align: Alignment) -> Self {
        Self {
            align,
            msg: msg.to_string(),
        }
    }
}

impl Widget for Msg {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wid_text_perc = self.msg.len() as f32 / area.width as f32;
        let constrains_text_box = if wid_text_perc < 0.75 {
            self.msg.len() as u16 + 2
        } else {
            (area.width as f32 * 0.75) as u16
        };

        // let c: &[Constraint] = &[Min(10), Percentage(constrains_text_box)];
        let c: &[Constraint] = &[Percentage(100), Length(constrains_text_box)];
        let [area, _] =
            Layout::vertical([Length(ILLUSTRATION_HEIGHT), Length(SPACER_HEIGHT)]).areas(area);
        let blocks = Layout::horizontal(c).split(area);
        let color = LENGTH_COLOR;
        let fg = Color::White;

        let title = format!("len {}", constrains_text_box);
        // let title = "me".to_string();
        let content = self.msg.to_string();
        // let content = format!("{}", self.msg.as_str());
        // let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::new(0, 0, 1, 0))
            // .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_type(ratatui::widgets::BorderType::Rounded);
        // .border_style(Style::reset().fg(color).reversed())
        // .style(Style::default().fg(fg).bg(color));
        Paragraph::new(content)
            .right_aligned()
            .block(block)
            .wrap(Wrap { trim: true })
            .render(blocks[1], buf);

        // Paragraph::new("").block(block).render(blocks[1], buf);
    }
}

// use std::usize;

// use ratatui::{
//     buffer::Buffer,
//     layout::{
//         Constraint::{self, Length, Max, Percentage},
//         Layout, Rect,
//     },
//     // style::{palette::tailwind, Color},
//     widgets::{
//         Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
//         Widget, Wrap,
//     },
// };

// // const LENGTH_COLOR: Color = tailwind::SLATE.c700;

// #[derive(Default, Clone, Debug)]
// pub struct MsgView {
//     messages: Vec<String>,
// }

// /// Tabs for the different examples
// ///
// /// The order of the variants is the order in which they are displayed.

// impl MsgView {
//     // pub fn update_max_scroll_offset(&mut self) {
//     //     //TODO:call this when drawing in app
//     //     if self.messages.len() > SCROLL_BOTTOM_NB_MSG_IN_SCOPE {
//     //         self.max_scroll_offset =
//     //             (self.messages.len() - SCROLL_BOTTOM_NB_MSG_IN_SCOPE) as u16 * HEIGHT_MSG_VIEW;
//     //     }
//     // }

//     pub fn get_nb_messages(&self) -> usize {
//         self.messages.len()
//     }

//     pub fn add_msg(&mut self, text: String) {
//         self.messages.push(text);
//     }

//     // pub fn down_one_msg(&mut self) {
//     //     //TODO:fuck this1! just go bottom??
//     //     self.scroll_offset = self
//     //         .scroll_offset
//     //         .saturating_add(HEIGHT_MSG_VIEW)
//     //         .min(self.max_scroll_offset);
//     // }
// }

// impl Widget for MsgView {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         // let height = self.messages.len() as u16 * HEIGHT_MSG_VIEW;
//         // let msg_area = Rect::new(0, 0, area.width, height + area.height);
//         // let mut msg_buf = Buffer::empty(msg_area);

//         // let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
//         // let content_area = if scrollbar_needed {
//         //     Rect {
//         //         width: msg_area.width - 1,
//         //         ..msg_area
//         //     }
//         // } else {
//         //     msg_area
//         // };

//         // let constraints: Vec<Constraint> = self
//         //     .messages
//         //     .iter()
//         //     .map(|_| Max(HEIGHT_MSG_VIEW)) //here have something more dynamic in height
//         //     .collect();

//         // // self.selected_tab.render(content_area, &mut demo_buf);
//         // let msg_grid = Layout::vertical(constraints.as_slice()).split(content_area);
//         // for (i, m) in self.messages.iter().enumerate() {
//         //     // Msg::new(&format!(
//         //     //     "area h {},  h msg {} need {} off {}",
//         //     //     area.height, height, scrollbar_needed, self.scroll_offset
//         //     // ))
//         //     Msg::new(m).render(msg_grid[i], &mut msg_buf);
//         // }

//         // let visible_content = msg_buf
//         //     .content
//         //     .into_iter()
//         //     .skip((msg_area.width * self.scroll_offset) as usize)
//         //     .take(area.area() as usize);
//         // for (i, cell) in visible_content.enumerate() {
//         //     let x = i as u16 % area.width;
//         //     let y = i as u16 / area.width;
//         //     buf[(area.x + x, area.y + y)] = cell;
//         // }

//         // if scrollbar_needed {
//         //     let mut state = ScrollbarState::new(self.max_scroll_offset as usize)
//         //         .position(self.scroll_offset as usize);
//         //     Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
//         // }
//     }
// }

// // struct Msg {
// //     msg: String,
// // }

// // impl Msg {
// //     fn new(msg: &str) -> Self {
// //         Self {
// //             msg: msg.to_string(),
// //         }
// //     }
// // }

// // impl Widget for Msg {
// //     fn render(self, area: Rect, buf: &mut Buffer) {
// //         // let wid_text_perc = self.msg.len() as f32 / area.width as f32;

// //         // let constrains_text_box = if wid_text_perc < 0.75 {
// //         //     self.msg.len() as u16 + 2
// //         // } else {
// //         //     //calculate the height
// //         //     (area.width as f32 * 0.75) as u16
// //         // };

// //         // // let c: &[Constraint] = &[Min(10), Percentage(constrains_text_box)];
// //         // let c: &[Constraint] = &[Percentage(100), Length(constrains_text_box)];
// //         // let [area, _] =
// //         //     Layout::vertical([Length(HEIGH_MSG_TEXT_VIEW), Length(HEIGHT_SPACER)]).areas(area);
// //         // let blocks = Layout::horizontal(c).split(area);
// //         // // let color = LENGTH_COLOR;
// //         // // let fg = Color::White;

// //         // let title = format!("len {}", constrains_text_box);
// //         // // let title = "me".to_string();
// //         // let content = self.msg.to_string();
// //         // // let content = format!("{}", self.msg.as_str());
// //         // // let text = format!("{title}\n{content}");
// //         // let block = Block::bordered()
// //         //     .title(title)
// //         //     .padding(Padding::new(0, 0, 1, 0))
// //         //     // .border_set(symbols::border::QUADRANT_OUTSIDE)
// //         //     .border_type(ratatui::widgets::BorderType::Rounded);
// //         // // .border_style(Style::reset().fg(color).reversed())
// //         // // .style(Style::default().fg(fg).bg(color));
// //         // Paragraph::new(content)
// //         //     .right_aligned()
// //         //     .block(block)
// //         //     .wrap(Wrap { trim: true })
// //         //     .render(blocks[1], buf);

// //         // // Paragraph::new("").block(block).render(blocks[1], buf);
// //     }
// // }

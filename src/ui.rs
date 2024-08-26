use std::fmt::format;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Masked, Span, Text},
    widgets::{Block, BorderType, Borders, ListItem, Paragraph, Widget, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.draw(frame);
    // app.render(frame.size(), frame.buffer_mut());
}

fn render_normal(app: &App, frame: &mut Frame) {

    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .margin(2)
    //     .constraints(
    //         [
    //             Constraint::Length(1),
    //             Constraint::Min(3),
    //             Constraint::Percentage(90),
    //         ]
    //         .as_ref(),
    //     )
    //     .split(frame.area());

    // let msg_help = vec![
    //     Span::raw("Press "),
    //     Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
    //     Span::raw("to exit, "),
    //     Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
    //     Span::raw("to start editing."),
    // ];
    // let msg_edit = vec![
    //     Span::raw("Press "),
    //     Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
    //     Span::raw("to edit, "),
    // ];
    // let style = Style::default().add_modifier(Modifier::RAPID_BLINK);
    // let text = Text::from(Line::from(msg_help)).style(style);
    // let help_message = Paragraph::new(text);
    // frame.render_widget(help_message, chunks[0]);

    // let width = chunks[0].width.max(3) - 3;
    // let scroll = app.input.visual_scroll(width as usize);
    // let input = Paragraph::new(app.input.value())
    //     .style(Style::default())
    //     .scroll((0, scroll as u16))
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Thick)
    //             .title_bottom(msg_edit)
    //             .title_alignment(Alignment::Right),
    //     )
    //     .wrap(Wrap { trim: true });
    // frame.render_widget(input, chunks[1]);
    // let messages: Vec<Paragraph> = app
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = Text::from(Span::raw(format!("{}: {}", i, m)));
    //         Paragraph::new(content).block(
    //             Block::default()
    //                 .borders(Borders::ALL)
    //                 .border_type(BorderType::Thick)
    //                 .title_top("me")
    //                 .title_alignment(Alignment::Left),
    //         )
    //     })
    //     .collect();
}

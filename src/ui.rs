use crate::app::App;
use ratatui::Frame;

/// Renders the user interface widgets.
//TODO:get rid of this file
pub fn render(app: &mut App, frame: &mut Frame) {
    app.draw(frame);
}

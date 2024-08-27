use crate::app::App;
use ratatui::Frame;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.draw(frame);
}

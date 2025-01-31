use ratatui::{
    style::Color,
    widgets::{
        canvas::{Canvas, Shape},
        Block, Widget,
    },
};
// 160 x 144
pub struct Screen;
impl Widget for &Screen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let frame = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Asdrubalino");
        Canvas::default()
            .block(frame)
            .x_bounds([160., 160.])
            .y_bounds([144.; 2])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| ctx.draw(&ScreenFrame { data: vec![] }))
            .render(area, buf);
    }
}
struct ScreenFrame {
    data: Vec<u8>,
}
impl Shape for ScreenFrame {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for x in 0..160 {
            for y in 0..144 {
                painter.paint(x, y, Color::Rgb(0, 0, 0));
            }
        }
    }
}

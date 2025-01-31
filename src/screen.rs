#![allow(unused, dead_code)]
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
        println!("{}, {}", area.height, area.width);
        let frame = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Asdrubalino");
        let ratio = area.width / area.height;
        Canvas::default()
            .block(frame)
            .x_bounds([160., 160.])
            .y_bounds([144.; 2])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| ctx.draw(&ScreenFrame))
            .render(area, buf);
    }
}
struct ScreenFrame;
impl Shape for ScreenFrame {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for y in 0..144 {
            for x in 0..160 {
                let r = (x as f32 / (159.) * 255.999) as u8;
                let g = (y as f32 / (143.) * 255.999) as u8;
                let b = 0;

                painter.paint(x, y, Color::Rgb(r, g, b));
                // painter.paint(x, y, Color::Rgb(0, 0, 0));
            }
        }
    }
}

use crate::widgets::common::Widget;
use wgpu_glyph::ab_glyph::PxScale;
use wgpu_glyph::{Extra, GlyphBrush, Section, Text};

pub struct BText {
    text: String,
    position: (f32, f32),
}

impl BText {
    pub fn new_static(text: &str, pos: (f32, f32)) -> Self {
        Self {
            text: String::from(text),
            position: pos,
        }
    }
    pub fn new(pos: (f32, f32)) -> Self {
        Self {
            text: String::new(),
            position: pos,
        }
    }
}

impl Widget for BText {
    fn draw(&self, brush: &mut GlyphBrush<()>) {
        let section: Section<Extra> = Section {
            screen_position: (30.0, 30.0),
            text: vec![
                Text::new(self.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(PxScale {
                        x: self.position.0,
                        y: self.position.1,
                    }),
            ],
            ..Default::default()
        };
        brush.queue(section);
    }

    fn update(&mut self) {}
}

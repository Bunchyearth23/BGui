use crate::globals::Globals;
use crate::widgets::common::{DrawCommand, Drawable, Widget};
use std::collections::HashMap;
use wgpu_glyph::ab_glyph::PxScale;
use wgpu_glyph::{Extra, Section, Text};

pub struct BText {
    text: String,
    position: (f32, f32),
    func: Option<Box<dyn FnMut(&mut Self, &mut HashMap<String, Globals>)>>,
}

impl BText {
    pub fn new(text: String, pos: (f32, f32)) -> Self {
        Self {
            text: text,
            position: pos,
            func: None,
        }
    }

    pub fn add_function<F: FnMut(&mut Self, &mut HashMap<String, Globals>) + 'static>(
        &mut self,
        in_func: F,
    ) {
        self.func = Some(Box::new(in_func))
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text_in: String) {
        self.text = text_in
    }
}

impl Drawable for BText {
    fn draw_command(&self) -> Vec<super::common::DrawCommand<'_>> {
        let mut commands = Vec::<DrawCommand>::new();
        let section: Section<Extra> = Section {
            screen_position: (self.position.0, self.position.1),
            text: vec![
                Text::new(self.text.as_str())
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(PxScale { x: 32.0, y: 32.0 }),
            ],
            ..Default::default()
        };
        commands.push(DrawCommand::Text(section));
        commands
    }
}

impl Widget for BText {
    fn drawable(&self) -> Box<dyn Drawable> {
        Box::new(Self {
            text: self.text.clone(),
            position: self.position,
            func: None,
        })
    }
    fn update(&mut self, global: &mut HashMap<String, Globals>) {
        if let Some(mut function) = self.func.take() {
            function(self, global);
            self.func = Some(function);
        }
    }
}

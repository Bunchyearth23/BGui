use crate::globals::Globals;
use crate::widgets::common::{DrawCommand, Drawable, Interactable, Widget};
use std::collections::HashMap;
use wgpu_glyph::ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use wgpu_glyph::{Extra, Section, Text};

pub struct BButton {
    text: String,
    position: (f32, f32),
    func: Option<Box<dyn FnMut(&mut Self, &mut HashMap<String, Globals>)>>,
    executable_func: Option<Box<dyn FnMut(&mut HashMap<String, Globals>)>>,
    start: (f32, f32),
    end: (f32, f32),
}

impl BButton {
    pub fn new(text: String, pos: (f32, f32)) -> Self {
        let font = FontArc::try_from_slice(include_bytes!(".././font/default.ttf")).unwrap();
        let scale = PxScale { x: 32.0, y: 32.0 };

        let mut text_width = 0.0;
        for c in text.chars() {
            let glyph = font.as_scaled(scale).h_advance(font.glyph_id(c));
            text_width += glyph;
        }

        let padding = 5.0;
        let text_height = 32.0;

        let start_pos_x = pos.0 - padding;
        let start_pos_y = pos.1 - padding;

        let end_pos_x = pos.0 + text_width + padding;
        let end_pos_y = pos.1 + text_height + padding;

        Self {
            text: text,
            position: pos,
            func: None,
            executable_func: None,
            start: (start_pos_x, start_pos_y),
            end: (end_pos_x, end_pos_y),
        }
    }

    pub fn add_function<F: FnMut(&mut Self, &mut HashMap<String, Globals>) + 'static>(
        &mut self,
        in_func: F,
    ) {
        self.func = Some(Box::new(in_func))
    }

    pub fn add_exec_function<F: FnMut(&mut HashMap<String, Globals>) + 'static>(
        &mut self,
        in_func: F,
    ) {
        self.executable_func = Some(Box::new(in_func))
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text_in: String) {
        self.text = text_in
    }
}

impl Drawable for BButton {
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

        commands.push(DrawCommand::Rect(
            (self.start.0, self.start.1),
            (self.end.0, self.end.1),
            [0.5, 0.5, 0.5, 1.0],
        ));
        commands.push(DrawCommand::Text(section));
        commands
    }
}

impl Widget for BButton {
    fn drawable(&self) -> Box<dyn Drawable> {
        Box::new(Self {
            text: self.text.clone(),
            position: self.position,
            func: None,
            executable_func: None,
            start: self.start,
            end: self.end,
        })
    }

    fn interactable(&self) -> Option<Box<dyn Interactable>> {
        Some(Box::new(Self {
            text: self.text.clone(),
            position: self.position,
            func: None,
            executable_func: None,
            start: self.start,
            end: self.end,
        }))
    }

    fn update(&mut self, global: &mut HashMap<String, Globals>) {
        if let Some(mut function) = self.func.take() {
            function(self, global);
            self.func = Some(function);
        }
    }

    fn execute_function(&mut self, global: &mut HashMap<String, Globals>) {
        if let Some(mut function) = self.executable_func.take() {
            function(global);
            self.executable_func = Some(function);
        }
    }
}

impl Interactable for BButton {
    fn in_interactable_zone(&self, mouse_position: (f32, f32)) -> bool {
        mouse_position.0 >= self.start.0
            && mouse_position.1 >= self.start.1
            && mouse_position.0 <= self.end.0
            && mouse_position.1 <= self.end.1
    }
}

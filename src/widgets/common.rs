use std::collections::HashMap;

use crate::prelude::*;
use wgpu_glyph::Section;

pub trait Widget {
    fn drawable(&self) -> Box<dyn Drawable>;
    fn interactable(&self) -> Option<Box<dyn Interactable>> {
        None
    }
    fn update(&mut self, _global: &mut HashMap<String, Globals>) {}
    fn execute_function(&mut self, _global: &mut HashMap<String, Globals>) {}
}

pub enum DrawCommand<'a> {
    Text(Section<'a>),
    Rect((f32, f32), (f32, f32)),
}

pub trait Drawable {
    fn draw_command(&self) -> Vec<DrawCommand<'_>>;
}

pub trait Interactable {
    fn in_interactable_zone(&self, _mouse_position: (f32, f32)) -> bool {
        false
    }
}

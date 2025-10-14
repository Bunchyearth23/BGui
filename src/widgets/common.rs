use std::collections::HashMap;

use crate::prelude::*;
use wgpu_glyph::Section;

pub trait Widget {
    fn drawable(&self) -> Box<dyn Drawable>;
    fn update(&mut self, _global: &mut HashMap<String, Globals>) {}
}

pub enum DrawCommand<'a> {
    Text(Section<'a>),
}

pub trait Drawable {
    fn draw_command(&self) -> Vec<DrawCommand<'_>>;
}

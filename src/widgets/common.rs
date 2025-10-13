use wgpu_glyph::GlyphBrush;

pub trait Widget {
    fn draw(&self, brush: &mut GlyphBrush<()>);
    fn update(&mut self) {}
}

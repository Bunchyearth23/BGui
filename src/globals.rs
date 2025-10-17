use crate::bgui::BGui;

pub enum Globals {
    Empty,
    Int(i32),
    Text(String),
    Float(f32),
    Other(Box<dyn Global>),
}

pub trait Global {
    fn get(&self, ui: &BGui) -> Box<dyn Global>;
    fn set(&self, id: String, ui: &mut BGui);
}

use std::any::Any;

pub enum Globals {
    Empty,
    Int(i32),
    Text(String),
    Float(f32),
    Other(Box<dyn Global>),
}

pub trait Global: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl dyn Global {
    pub fn get_any<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

use std::{any::Any, collections::HashMap};

pub enum Globals {
    Empty,
    Int(i32),
    Text(String),
    Float(f32),
    Other(Box<dyn Global>),
}

pub trait Global: Any {
    fn as_any(&self) -> &dyn Any;
    fn set(self, id: String, globals: &mut HashMap<String, Globals>);
}

impl dyn Global {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

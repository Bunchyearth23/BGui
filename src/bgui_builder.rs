use std::rc::Rc;

use pollster::FutureExt;
use wgpu::{
    Adapter, DeviceDescriptor, Instance, InstanceDescriptor, RequestAdapterOptionsBase,
    SurfaceTarget,
};
use winit::{
    dpi::{LogicalSize, Size},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::prelude::*;

pub enum WinMode {
    FULL,
    WINDOW,
}

pub struct BGuiBuilder {
    x: Option<u32>,
    y: Option<u32>,
    title: Option<String>,
    mode: Option<WinMode>,
    resizable: bool,
}

impl BGuiBuilder {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            title: None,
            mode: None,
            resizable: false,
        }
    }

    pub fn width(mut self, x: u32) -> Self {
        self.x = Some(x);
        self
    }

    pub fn height(mut self, y: u32) -> Self {
        self.y = Some(y);
        self
    }

    pub fn mode(mut self, mode: WinMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    pub fn build(self, events: ActiveEventLoop) -> BGui {
        let mut attrib: WindowAttributes = Window::default_attributes();

        attrib.resizable = self.resizable;
        attrib.title = self.title.unwrap_or("No Title".to_string());

        attrib = attrib.with_inner_size(Size::Logical(LogicalSize::new(
            self.x.expect("Missing Width") as f64,
            self.y.expect("Missing Height") as f64,
        )));

        let instance = Instance::new(&InstanceDescriptor::from_env_or_default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase::default())
            .block_on()
            .expect("No compatible Adapters");
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .block_on()
            .expect("No compatible Devices");

        let window = events
            .create_window(attrib)
            .expect("Failed to create Window");
        let surface = instance
            .create_surface(window)
            .expect("Failed to create Surface");

        BGui {
            instance: instance,
            adapter: adapter,
            device: device,
            queue: queue,
            surface: surface,
        }
    }
}

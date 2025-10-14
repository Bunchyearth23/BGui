use crate::widgets::common::Widget;
use std::collections::HashMap;
use std::process;

use wgpu::util::StagingBelt;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{Surface, SurfaceConfiguration};
use wgpu_glyph::ab_glyph::FontArc;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;
use winit::{self, event_loop::EventLoop};

use crate::globals::Globals;

pub struct BGui {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) attributes: WindowAttributes,
    pub(crate) glyph_brush: Option<wgpu_glyph::GlyphBrush<()>>,
    pub(crate) staging_belt: StagingBelt,

    pub(crate) widgets: Vec<Box<dyn Widget>>,
    pub(crate) globals: HashMap<String, Globals>,

    pub(crate) surface: Option<Surface<'static>>,
    pub(crate) surface_config: Option<SurfaceConfiguration>,
}

impl<'a> BGui {
    pub fn run(mut self) -> Result<(), EventLoopError> {
        let events = EventLoop::new().expect("Failed to start EventLoop");
        events.run_app(&mut self)
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(self.attributes.clone())
            .expect("Failed to create window");

        let surface = self
            .instance
            .create_surface(window)
            .expect("Failed to create Surface");
        let config = surface
            .get_default_config(&self.adapter, 1280, 720)
            .expect("Missing config");
        surface.configure(&self.device, &config);
        self.surface = Some(surface);
        self.surface_config = Some(config);
        let font = FontArc::try_from_slice(include_bytes!("./font/default.ttf"))
            .expect("Cannot load font");
        let brush_builder = wgpu_glyph::GlyphBrushBuilder::using_font(font);
        let brush = brush_builder.build(
            &self.device,
            self.surface
                .as_ref()
                .unwrap()
                .get_capabilities(&self.adapter)
                .formats[0],
        );
        self.glyph_brush = Some(brush);
    }

    fn reconfigure_surface(&mut self, width: u32, height: u32) {
        if let Some(surface) = &self.surface {
            let config = surface
                .get_default_config(&self.adapter, width, height)
                .expect("Missing config");
            surface.configure(&self.device, &config);
            self.surface_config = Some(config);
        }
    }

    fn render(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        match surface.get_current_texture() {
            Ok(frame) => {
                let mut view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.2,
                                g: 0.2,
                                b: 0.2,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                for widget in &self.widgets {
                    let drawable = widget.drawable();
                    for cmd in drawable.draw_command() {
                        match cmd {
                            crate::prelude::common::DrawCommand::Text(section) => {
                                self.glyph_brush.as_mut().unwrap().queue(section)
                            }
                        }
                    }
                }

                self.glyph_brush
                    .as_mut()
                    .unwrap()
                    .draw_queued(
                        &self.device,
                        &mut self.staging_belt,
                        &mut encoder,
                        &mut view,
                        self.surface_config.as_ref().unwrap().width,
                        self.surface_config.as_ref().unwrap().height,
                    )
                    .expect("Failed to draw text");

                self.staging_belt.finish();
                self.queue.submit(std::iter::once(encoder.finish()));
                frame.present();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn update_widgets(&mut self) {
        for x in self.widgets.iter_mut() {
            x.update(&mut self.globals);
        }
    }

    pub fn insert_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget));
    }

    pub fn insert_global(&mut self, id: String, global: Globals) {
        self.globals.insert(id, global);
    }
}

impl ApplicationHandler for BGui {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.init(&event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                process::exit(0);
            }
            WindowEvent::RedrawRequested => {
                self.update_widgets();
                self.render();
            }
            WindowEvent::Resized(size) => {
                let (x, y) = (size.width, size.height);
                self.reconfigure_surface(x, y);
            }
            _ => (),
        }
    }
}

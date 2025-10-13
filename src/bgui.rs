use std::process;

use wgpu::Surface;
use wgpu::wgt::TextureViewDescriptor;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;
use winit::{self, event_loop::EventLoop};

pub struct BGui {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) attributes: WindowAttributes,

    pub(crate) surface: Option<Surface<'static>>,
}

impl BGui {
    pub fn run(mut self) -> Result<(), EventLoopError> {
        let events = EventLoop::new().expect("Failed to start EventLoop");
        events.run_app(&mut self)
    }

    fn _render(&mut self) {}
    fn init_surface(&mut self, event_loop: &ActiveEventLoop) {
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
    }
    fn clear_background(&mut self) {
        let surface = self.surface.as_mut().unwrap();
        match surface.get_current_texture() {
            Ok(frame) => {
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                {
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
                }

                self.queue.submit(std::iter::once(encoder.finish()));
                frame.present();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn reconfigure_surface(&mut self, width: u32, height: u32) {
        if let Some(surface) = &self.surface {
            let config = surface
                .get_default_config(&self.adapter, width, height)
                .expect("Missing config");
            surface.configure(&self.device, &config);
        }
    }
}

impl ApplicationHandler for BGui {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.init_surface(&event_loop);
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
                self.clear_background();
            }
            WindowEvent::Resized(size) => {
                let (x, y) = (size.width, size.height);
                self.reconfigure_surface(x, y);
            }
            _ => (),
        }
    }
}

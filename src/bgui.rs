use std::process;

use wgpu::rwh::HasDisplayHandle;
use wgpu::{Surface, SurfaceConfiguration, TextureUsages};
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::{self, event_loop::EventLoop};

pub struct BGui {
    //pub(crate) win: Window,
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,

    pub(crate) surface: Surface<'static>,
}

impl BGui {
    pub fn run(mut self) -> Result<(), EventLoopError> {
        let events = EventLoop::new().expect("Failed to start EventLoop");
        events.run_app(&mut self)
    }

    fn render(&mut self) {}
    fn init_surface(&mut self) {
        let config = self
            .surface
            .get_default_config(&self.adapter, 1280, 720)
            .expect("Missing config");
        self.surface.configure(&self.device, &config);
    }
}

impl ApplicationHandler for BGui {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.init_surface();
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
                let mut encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            ..Default::default()
                        });
                {
                    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: todo!(),
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
            }
            _ => (),
        }
    }
}

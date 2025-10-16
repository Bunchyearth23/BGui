use crate::prelude::common::DrawCommand;
use crate::widgets::common::Widget;
use std::collections::HashMap;
use std::process;

use wgpu::util::{BufferInitDescriptor, DeviceExt, StagingBelt};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BlendState, BufferUsages, ColorTargetState, ColorWrites,
    FragmentState, MultisampleState, PipelineCompilationOptions, PrimitiveState, RenderPipeline,
    ShaderModuleDescriptor, Surface, SurfaceConfiguration, VertexState,
};
use wgpu_glyph::ab_glyph::FontArc;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::{ElementState, MouseButton, WindowEvent};
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
    pub(crate) pipelines: HashMap<String, RenderPipeline>,

    pub(crate) surface: Option<Surface<'static>>,
    pub(crate) surface_config: Option<SurfaceConfiguration>,

    pub(crate) mouse_position: (f32, f32),
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

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                            DrawCommand::Rect(start, end) => {
                                let pipeline = self
                                    .pipelines
                                    .get("rectangle_pipeline")
                                    .expect("Failed to retreive Pipeline");

                                let color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

                                let color_buffer =
                                    self.device.create_buffer_init(&BufferInitDescriptor {
                                        label: None,
                                        contents: bytemuck::cast_slice(&[color]),
                                        usage: BufferUsages::UNIFORM,
                                    });

                                let bind_group =
                                    self.device.create_bind_group(&BindGroupDescriptor {
                                        label: None,
                                        layout: &pipeline.get_bind_group_layout(0),
                                        entries: &[BindGroupEntry {
                                            binding: 0,
                                            resource: color_buffer.as_entire_binding(),
                                        }],
                                    });

                                render_pass.set_pipeline(pipeline);
                                render_pass.set_bind_group(0, &bind_group, &[]);

                                let (x, y) = (
                                    self.surface_config.as_ref().unwrap().width,
                                    self.surface_config.as_ref().unwrap().height,
                                );

                                let x0 = start.0 / (x as f32) * 2.0 - 1.0;
                                let y0 = 1.0 - start.1 / (y as f32) * 2.0;
                                let x1 = end.0 / (x as f32) * 2.0 - 1.0;
                                let y1 = 1.0 - end.1 / (y as f32) * 2.0;

                                let vertices = [
                                    ButtonVertex { position: [x0, y0] },
                                    ButtonVertex { position: [x1, y0] },
                                    ButtonVertex { position: [x0, y1] },
                                    ButtonVertex { position: [x1, y1] },
                                    ButtonVertex { position: [x0, y1] },
                                    ButtonVertex { position: [x1, y0] },
                                ];

                                let vertex_buffer =
                                    self.device.create_buffer_init(&BufferInitDescriptor {
                                        label: None,
                                        contents: bytemuck::cast_slice(&vertices),
                                        usage: BufferUsages::VERTEX,
                                    });

                                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                                render_pass.draw(0..6, 0..1);
                            }
                            DrawCommand::Text(section) => {
                                self.glyph_brush.as_mut().unwrap().queue(section)
                            }
                        }
                    }
                }

                drop(render_pass);

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

    fn init_shaders(&mut self) {
        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/rectangle.wgsl").into()),
        });

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, // accessible dans les deux shaders
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let button_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let button_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&button_pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("v_main"),
                    buffers: &[ButtonVertex::desc()],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("f_main"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: self.surface_config.as_ref().unwrap().format,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        self.pipelines
            .insert("rectangle_pipeline".into(), button_pipeline);
    }

    fn interact_widget(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => (),
                ElementState::Released => {
                    for widget in self.widgets.iter_mut() {
                        match widget.interactable() {
                            Some(x) => {
                                if x.in_interactable_zone(self.mouse_position) {
                                    widget.execute_function(&mut self.globals);
                                };
                            }
                            None => (),
                        }
                    }
                }
            },
            MouseButton::Right => (),
            MouseButton::Middle => (),
            MouseButton::Back => (),
            MouseButton::Forward => (),
            MouseButton::Other(_) => (),
        }
    }
}

impl ApplicationHandler for BGui {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.init(&event_loop);
        self.init_shaders();
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
            #[allow(unused)]
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                self.interact_widget(button, state);
            }
            #[allow(unused)]
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => self.mouse_position = (position.x as f32, position.y as f32),
            WindowEvent::Resized(size) => {
                let (x, y) = (size.width, size.height);
                self.reconfigure_surface(x, y);
            }
            _ => (),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ButtonVertex {
    position: [f32; 2], // x et y
}

impl ButtonVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ButtonVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // correspond à @location(0) dans WGSL
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

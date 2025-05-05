// use std::iter;

// use winit::{
//     application::ApplicationHandler, event::*, event::WindowEvent, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
// };

// struct State<'a> {
//     surface: wgpu::Surface<'a>,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: winit::dpi::PhysicalSize<u32>,
//     window: &'a Window,
//     surface_configured: bool
// }

// impl<'a> State<'a> {
//     async fn new(window: &'a Window) -> State<'a> {
//         let size = window.inner_size();

//         // The instance is a handle to our GPU
//         // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
//         let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
//             backends: wgpu::Backends::PRIMARY,
//             ..Default::default()
//         });

//         let surface = instance.create_surface(window).unwrap();

//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::default(),
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap();

//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     label: None,
//                     required_features: wgpu::Features::empty(),
//                     required_limits: wgpu::Limits::default(),
//                     memory_hints: Default::default(),
//                     trace: wgpu::Trace::Off,
//                 }
//             )
//             .await
//             .unwrap();

//         let surface_caps = surface.get_capabilities(&adapter);
//         // Shader code in this tutorial assumes an Srgb surface texture. Using a different
//         // one will result all the colors comming out darker. If you want to support non
//         // Srgb surfaces, you'll need to account for that when drawing to the frame.
//         let surface_format = surface_caps
//             .formats
//             .iter()
//             .copied()
//             .find(|f| f.is_srgb())
//             .unwrap_or(surface_caps.formats[0]);
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface_format,
//             width: size.width,
//             height: size.height,
//             present_mode: surface_caps.present_modes[0],
//             alpha_mode: surface_caps.alpha_modes[0],
//             desired_maximum_frame_latency: 2,
//             view_formats: vec![],
//         };

//         Self {
//             surface,
//             device,
//             queue,
//             config,
//             size,
//             window,
//             surface_configured: false
//         }
//     }

//     #[allow(unused_variables)]
//     fn input(&mut self, event: &WindowEvent) -> bool {
//         false
//     }

//     pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         if new_size.width > 0 && new_size.height > 0 {
//             self.size = new_size;
//             self.config.width = new_size.width;
//             self.config.height = new_size.height;
//             self.surface.configure(&self.device, &self.config);
//         }
//     }

//     fn update(&mut self) {}

//     fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output = self.surface.get_current_texture()?;
//         let view = output
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());

//         let mut encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                 label: Some("Render Encoder"),
//             });

//         {
//             let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(wgpu::Color {
//                             r: 0.1,
//                             g: 0.2,
//                             b: 0.3,
//                             a: 1.0,
//                         }),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//                 occlusion_query_set: None,
//                 timestamp_writes: None,
//             });
//         }

//         self.queue.submit(iter::once(encoder.finish()));
//         output.present();

//         Ok(())
//     }
// }

// impl <'a>ApplicationHandler for State<'a> {
//     #[allow(unused_variables)]
//     fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
//     }

//     fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
//         let window_attributes = WindowAttributes::default();
//         self.window = match event_loop.create_window(window_attributes) {
//             Ok(window) => Some(window),
//             Err(err) => {
//                 eprintln!("error creating window: {err}");
//                 event_loop.exit();
//                 return;
//             },
//         }
//     }

//     fn window_event(
//             &mut self,
//             event_loop: &winit::event_loop::ActiveEventLoop,
//             window_id: winit::window::WindowId,
//             event: WindowEvent,
//         ) {
//         if window_id == self.window.id() {
//             if !self.input(&event) {
//                 // UPDATED!
//                 match event {
//                     WindowEvent::CloseRequested
//                     | WindowEvent::KeyboardInput {
//                         event:
//                             KeyEvent {
//                                 state: ElementState::Pressed,
//                                 physical_key: PhysicalKey::Code(KeyCode::Escape),
//                                 ..
//                             },
//                         ..
//                     } => event_loop.exit(),
//                     WindowEvent::Resized(physical_size) => {
//                         log::info!("physical_size: {physical_size:?}");
//                         self.surface_configured = true;
//                         self.resize(physical_size);
//                     }
//                     WindowEvent::RedrawRequested => {
//                         self.window.request_redraw();

//                         if !self.surface_configured {
//                             return;
//                         }

//                         self.update();
//                         match self.render() {
//                             Ok(_) => {}

//                             Err(
//                                 wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
//                             ) => self.resize(self.size),

//                             Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
//                                 log::error!("OutOfMemory");
//                                 event_loop.exit();
//                             }
//                             Err(wgpu::SurfaceError::Timeout) => {
//                                 log::warn!("Surface timeout")
//                             }
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//         }

//     }
// }


// pub async fn run() {
//     let event_loop = EventLoop::new().unwrap();
//     let window = event_loop.create_window(window_attributes)
//     // let window = WindowBuilder::new().build(&event_loop).unwrap();

//     // let mut state = State::new(&window).await;
//     // let mut surface_configured = false;
//     let app = State::new(&window).await;

//     let _ = event_loop.run_app(&mut app).unwrap();
// }
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent, DeviceEvent, DeviceId};
use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use futures::executor::block_on;

#[derive(Default)]
struct App {
    window_state: Option<WindowState>,
}

struct WindowState {
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    device: wgpu::Device,
    window: Arc<Window>,
    instance: wgpu::Instance,
}

impl WindowState {
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl ApplicationHandler for App {
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let win_attrs = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(win_attrs).unwrap());
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })).unwrap();

        let (device, queue) = block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                }
            )).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        window.request_redraw();
        self.window_state = Some(WindowState {
            surface,
            queue,
            config,
            size,
            device,
            window,
            instance
        })

    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        // `unwrap` is fine, the window will always be available when
        // receiving a window event.
        // Handle window event.

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            _ => (),
        }

        let window_state = self.window_state.as_mut().unwrap();
        window_state.render();

    }
    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        // Handle window event.
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut state = App::default();
    let _ = event_loop.run_app(&mut state);
}
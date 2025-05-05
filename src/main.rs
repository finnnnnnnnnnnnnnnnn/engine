use std::process::Output;

use wgpu::wgc::instance;
use wgpu::Instance;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
struct App<'a> {
    surface: Option<wgpu::Surface<'a>>,
    queue: Option<wgpu::Queue>,
    device: Option<wgpu::Device>,
    window: Option<Window>,
    instance: Option<Instance>,
}

impl<'a> App<'a> {
    // fn default() -> App<'a> {
    //     let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
    //         backends: wgpu::Backends::PRIMARY,
    //         ..Default::default()
    //     });

    //     Self {
    //         instance,
    //         ..Default::default()
    //     }
    // }
    fn default() -> App<'a> {
        Self {
            ..Default::default()
        }
    }

    fn render(&mut self) {
        let output = self.surface.as_ref().unwrap().get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .as_ref()
            .unwrap()
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

        self.queue.as_ref().unwrap().submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed<'a>(&mut self, event_loop: &ActiveEventLoop) {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::PRIMARY,
                    ..Default::default()
                });
        let window_attributes = WindowAttributes::default();
        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        let test= self.window.as_ref().unwrap();
        self.surface = Some(instance.create_surface(test).unwrap());
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
                self.render()
            }
            _ => (),
        }
    }

}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}
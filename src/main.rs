use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowId};

use crate::frame_counter::FrameCounter;
use crate::renderer::renderer::Renderer;
use crate::world::World;

mod frame_counter;
mod renderer;
mod utils;
mod world;

#[derive(Default)]
struct App {
  window: Option<Arc<Window>>,
  frame_counter: FrameCounter,
  renderer: Option<Renderer>,
  is_vertex_data_uploaded: bool,
  world: Option<World>,
}

impl App {
  fn init(&mut self, window: Arc<Window>) {
    self.world = Some(World::new(&window.clone()));
    self.renderer = Some(pollster::block_on(Renderer::new(window)));
    self
      .renderer
      .as_mut()
      .unwrap()
      .create_shape_render_pipeline();
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = Some(Arc::new(
      event_loop
        .create_window(
          Window::default_attributes().with_fullscreen(Some(Fullscreen::Borderless(None))),
        )
        .unwrap(),
    ));

    if self.renderer.is_none() {
      match self.window.clone() {
        Some(window) => self.init(window),
        None => panic!("Window creation failed when trying to resume"),
      }
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed; stopping");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        // Redraw the application.
        //
        // It's preferable for applications that do not render continuously to render in
        // this event rather than in AboutToWait, since rendering in here allows
        // the program to gracefully handle redraws requested by the OS.

        self.frame_counter.update(true);
        // Draw.
        if let Some(window) = self.window.as_ref()
          && let Some(renderer) = self.renderer.as_mut()
        {
          if !self.is_vertex_data_uploaded {
            renderer.upload_square();
          }
          renderer.render(window, self.world.as_ref().unwrap());
          self.world.as_mut().unwrap().update(window);
        }

        // Queue a RedrawRequested event.
        //
        // You only need to call this if you've determined that you need to redraw in
        // applications which do not always need to. Applications that redraw continuously
        // can render here instead.
        self.window.as_ref().unwrap().request_redraw();
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

  // ControlFlow::Wait pauses the event loop if no events are available to process
  // This is ideal for non-game applications that only update in response to user
  // input, and uses significantly less power/CPU time than ControlFlow::Poll.
  // event_loop.set_control_flow(ControlFlow::Wait);

  let mut app = App::default();

  let _ = event_loop.run_app(&mut app);
}

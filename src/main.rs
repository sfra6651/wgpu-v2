use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowId};

use crate::frame_counter::FrameCounter;
use crate::player_controller::update_player;
use crate::renderer::renderer::Renderer;
use crate::world::World;

mod camera;
mod entity;
mod frame_counter;
mod player_controller;
mod renderer;
mod utils;
mod world;

const GAME_UPDATES_PER_SEC: f32 = 1.0 / 60.0; // seconds per fixed simulation step (60 Hz)

#[derive(Default)]
struct App {
  window: Option<Arc<Window>>,
  frame_counter: FrameCounter,
  renderer: Option<Renderer>,
  world: Option<World>,
  last_update: Option<Instant>,
  accumulator: f32,
}

impl App {
  fn init(&mut self, window: Arc<Window>) {
    self.world = Some(World::new());
    self.renderer = Some(pollster::block_on(Renderer::new(window)));
    self.renderer.as_mut().unwrap().create_pipelines();

    let size = self.world.as_ref().unwrap().size;
    self.renderer.as_mut().unwrap().upload_grid(size, 1.0); // 1.0-unit spacing
    self.renderer.as_mut().unwrap().upload_square();
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

        // Advance the fixed-timestep accumulator by real elapsed time.
        let now = Instant::now();
        let frame_dt = self
          .last_update
          .map(|t| now.duration_since(t).as_secs_f32())
          .unwrap_or(0.0);
        self.last_update = Some(now);
        self.accumulator += frame_dt;

        // Draw.
        if let Some(window) = self.window.as_ref()
          && let Some(renderer) = self.renderer.as_mut()
        {
          // Fixed-rate update, capped at one step per frame (no catch-up queue).
          if self.accumulator >= GAME_UPDATES_PER_SEC {
            if let Some(world) = self.world.as_mut() {
              world.update();
            }
            self.accumulator -= GAME_UPDATES_PER_SEC;

            // Still a full step behind? We're running below 60 Hz — drop the
            // backlog and run in slow motion rather than queuing updates.
            if self.accumulator >= GAME_UPDATES_PER_SEC {
              self.accumulator = 0.0;
            }
          }

          renderer.render(window, self.world.as_ref().unwrap());
        }

        // Queue a RedrawRequested event.
        //
        // You only need to call this if you've determined that you need to redraw in
        // applications which do not always need to. Applications that redraw continuously
        // can render here instead.
        self.window.as_ref().unwrap().request_redraw();
      }
      WindowEvent::KeyboardInput {
        device_id: _,
        event,
        is_synthetic: _,
      } => {
        if let Some(world) = self.world.as_mut() {
          update_player(world, &event);
        }
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

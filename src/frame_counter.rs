use std::time::Instant;

#[derive(Default)]
pub struct FrameCounter {
  count: usize,
  last_recorded_at: Option<Instant>,
}

impl FrameCounter {
  pub fn update(&mut self, log_count: bool) {
    match self.last_recorded_at {
      Some(instant) => {
        self.count += 1;
        let elapsed = instant.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
          if log_count {
            println!("fps: {}", self.count);
          }
          self.count = 0;
          self.last_recorded_at = Some(Instant::now());
        }
      }
      None => {
        self.count += 1;
        self.last_recorded_at = Some(Instant::now());
      }
    }
  }
}

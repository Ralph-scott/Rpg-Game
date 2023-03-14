use std::time::{Duration, Instant};

pub struct Timer {
    now: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            now: Instant::now(),
            duration,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.now.elapsed()
    }

    pub fn finished(&self) -> bool {
        self.elapsed() >= self.duration
    }

    pub fn reset(&mut self) {
        self.now = Instant::now();
    }
}

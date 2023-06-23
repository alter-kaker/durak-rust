use std::time::{Duration, Instant};

pub struct Timer {
    runtime: Duration,
    clock: Instant,
    dt: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            runtime: Duration::ZERO,
            dt: Duration::ZERO,
            clock: Instant::now(),
        }
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn runtime(&self) -> Duration {
        self.runtime
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.dt = now - self.clock;
        self.clock = now;
        self.runtime += self.dt;
    }
}

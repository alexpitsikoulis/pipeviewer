use std::time::{Instant, Duration};


pub struct Timer {
    pub start: Instant,
    pub last_instant: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Timer {
            start: now,
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(100),
            countdown: Duration::default(),
            ready: true,
        }
    }
    
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, remaining) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (remaining / 60, remaining % 60);
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds) 
    }
}

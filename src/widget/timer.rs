pub enum TimerMode {
    Countdown,
    Countup,
}

pub struct TimerState {
    pub mode: TimerMode,
    pub initial: u64,
    pub remaining: u64,
    pub running: bool,
}

impl TimerState {
    pub fn new_countdown(duration_seconds: u64) -> Self {
        Self {
            mode: TimerMode::Countdown,
            initial: duration_seconds,
            remaining: duration_seconds,
            running: false,
        }
    }

    pub fn new_countup() -> Self {
        Self {
            mode: TimerMode::Countup,
            initial: 0,
            remaining: 0,
            running: false,
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }
        match self.mode {
            TimerMode::Countdown => {
                self.remaining = self.remaining.saturating_sub(1);
                if self.remaining == 0 {
                    self.running = false;
                }
            }
            TimerMode::Countup => {
                self.remaining = self.remaining.saturating_add(1);
            }
        }
    }

    pub fn toggle(&mut self) {
        if matches!(self.mode, TimerMode::Countdown) && self.remaining == 0 {
            return;
        }
        self.running = !self.running;
    }

    pub fn reset(&mut self) {
        self.running = false;
        self.remaining = match self.mode {
            TimerMode::Countdown => self.initial,
            TimerMode::Countup => 0,
        };
    }

    pub fn format(&self) -> String {
        let mins = self.remaining / 60;
        let secs = self.remaining % 60;
        format!("{mins:02}:{secs:02}")
    }
}

use std::fmt;
pub struct Duration {
    name: String,
    duration: u64,
    current_duration: u64,
    break_duration: u64,
    working: bool,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = self.current_duration / 60;
        let seconds = self.current_duration % 60;
        write!(f, "{}:{:02}", minutes, seconds)
    }
}

impl Duration {
    pub fn new(name: String, duration_mins: u64, break_duration_mins: u64) -> Self {
        let duration = duration_mins * 60;
        let break_duration = break_duration_mins * 60;
        Self {
            name,
            duration,
            current_duration: duration,
            break_duration,
            working: true,
        }
    }

    pub fn dec(&mut self) {
        self.current_duration -= 1;

        if self.current_duration == 0 {
            self.change_period();
        }
    }

    pub fn change_period(&mut self) {
        self.working = !self.working;
        if self.working {
            self.current_duration = self.duration;
        } else {
            self.current_duration = self.break_duration;
        }
    }

    pub fn get_status(&self) {
        if self.working {
            println!("✏️ Doing {}, {} minutes remaining", self.name, self);
        } else {
            println!("😴 Taking a break, {} minutes remaining", self);
        }
    }
}

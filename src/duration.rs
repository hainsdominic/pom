use notify_rust::Notification;
use std::fmt;

pub struct Duration {
    name: String,
    work_duration: u64,
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
    pub fn new(name: String, work_duration_mins: u64, break_duration_mins: u64) -> Self {
        let work_duration = work_duration_mins * 60;
        let break_duration = break_duration_mins * 60;
        Self {
            name,
            work_duration,
            current_duration: work_duration,
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
        let notification_msg: String;
        if self.working {
            self.current_duration = self.work_duration;
            notification_msg = String::from("Time to work!");
        } else {
            self.current_duration = self.break_duration;
            notification_msg = String::from("Break time!");
        }
        Notification::new()
            .summary("Pom")
            .body(&notification_msg)
            .show()
            .unwrap();
    }

    pub fn get_status(&self) {
        if self.working {
            println!("✏️ Doing {}, {} minutes remaining", self.name, self);
        } else {
            println!("😴 Taking a break, {} minutes remaining", self);
        }
    }
}

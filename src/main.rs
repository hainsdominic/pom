use clap::Parser;
use duration::Duration;
use std::{thread, time};
mod duration;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Name of task
    #[clap(short, long, default_value = "a task")]
    name: String,

    /// Focus period duration (Minutes)
    #[clap(short, long, default_value_t = 25)]
    duration: u64,

    /// Break period duration (Minutes)
    #[clap(short, long, default_value_t = 5)]
    break_duration: u64,
}

fn main() {
    let args = Args::parse();
    let mut duration = Duration::new(args.name, args.duration, args.break_duration);

    loop {
        duration.get_status();

        print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
        duration.dec();
        thread::sleep(time::Duration::from_secs(1));
    }
}

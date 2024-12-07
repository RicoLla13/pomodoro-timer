use std::{thread::sleep, time::Duration};
use std::io;

pub struct PomodoroTimer {
    pub pomodoro_duration: u64,
    pub break_duration: u64,
    pub num_pomodoros: u64,
}

impl PomodoroTimer {
    pub fn new(pomodoro_duration: u64, break_duration: u64, num_pomodoros: u64) -> Self {
        PomodoroTimer {
            pomodoro_duration,
            break_duration,
            num_pomodoros,
        }
    }

    fn countdown(&self, duration: u64, label: &str) {
        for i in (1..=duration).rev() {
            print!("\r{}: {:>2} seconds left", label, i);
            io::Write::flush(&mut io::stdout()).unwrap();
            sleep(Duration::from_secs(1));
        }
        println!("\n{} is over!", label);
    }

    fn run_pomodoro(&self) {
        self.countdown(self.pomodoro_duration, "Pomodoro session"); 
    }

    fn run_break(&self) {
        self.countdown(self.break_duration, "Break"); 
    }

    pub fn start(&self) {
        for i in 1..=self.num_pomodoros {
            println!("Pomodoro #{} started!", i);
            self.run_pomodoro();
            if i != self.num_pomodoros {
                println!("Take a break!");
                self.run_break();
            }
        }
    }
}

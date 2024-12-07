mod pomodoro;

use std::io;
use pomodoro::PomodoroTimer;

fn main() {
    println!("=== Welcome to Pomodoro timer! ===\n");

    let pomodoro_duration = get_input("Enter Pomodoro duration (in seconds): ");
    let break_duration = get_input("Enter Break duration (in seconds): ");
    let num_pomodoros = get_input("Enter the number of Pomodoros: ");

    let timer = PomodoroTimer::new(pomodoro_duration, break_duration, num_pomodoros);

    println!("Timer started for {} Pomodoros.", num_pomodoros);
    timer.start();
}

fn get_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("[*] Failed to read line!");

        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[*] Invalid input. Please enter value again");
                continue;
            }
        };

        break input;
    }
}

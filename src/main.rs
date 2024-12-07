use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("=== Welcome to Pomodoro timer! ===\n");

    let _duration = loop {
        println!("Enter the time duration (in seconds): ");

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
    };

    println!("Timer started for {} seconds!", _duration);

    for i in (1..=_duration).rev() {
        print!("\rTime remaining: {:>2} seconds", i);
        io::Write::flush(&mut io::stdout()).unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\nTime's up!");
}

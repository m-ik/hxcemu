mod alu;
mod cpu;
mod hack_computer;
mod ram;
mod rom;

use std::env;
use std::process::ExitCode;
use std::thread;

use hack_computer::HackComputer;

fn main() -> ExitCode {
    println!("hack emulator");

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <file>", args[0]);
        return ExitCode::from(1);
    }

    let file_path = &args[1];

    let mut hack_computer = match HackComputer::new(file_path) {
        Ok(hack_computer) => hack_computer,
        Err(err) => {
            eprintln!("error: failed to initialise the computer: {}", err);
            return ExitCode::from(1);
        }
    };

    let hack_runner = thread::spawn(move || hack_computer.run());

    match hack_runner.join() {
        Ok(Ok(())) => println!("done"),
        Ok(Err(e)) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
        Err(_) => {
            eprintln!("error: emulator thread panicked");
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}

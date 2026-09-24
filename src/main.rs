mod alu;
mod app;
mod common;
mod cpu;
mod disas;
mod hack_computer;
mod isa;
mod ram;
mod rom;
mod tui;

use std::{
    process::ExitCode,
    sync::{
        mpsc::{self, Receiver, RecvTimeoutError},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use clap::Parser;

use crate::{common::Command, hack_computer::HackComputer, tui::Tui};

#[derive(Parser)]
#[command(about = "hack computer emulator")]
struct Args {
    program: String,

    #[arg(short, long, default_value_t = 10)]
    tick_ms: u64,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let hack_computer = match HackComputer::new(args.program) {
        Ok(hack_computer) => hack_computer,
        Err(err) => {
            eprintln!("error: failed to initialise the computer: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let hack_computer = Arc::new(Mutex::new(hack_computer));
    let (tx, rx) = mpsc::channel::<Command>();

    let hack_computer_clone = Arc::clone(&hack_computer);
    let tick_interval = Duration::from_millis(args.tick_ms);
    let emulator_handle = thread::spawn(move || {
        emulator_thread(hack_computer_clone, rx, tick_interval);
    });

    let mut tui = match Tui::new() {
        Ok(tui) => tui,
        Err(e) => {
            eprintln!("error: failed to initialise the TUI: {}", e);
            return ExitCode::FAILURE;
        }
    };
    match tui.enter() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: failed to enter TUI: {}", e);
            return ExitCode::FAILURE;
        }
    }
    let res = tui.run(&hack_computer, &tx);
    match tui.exit() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: failed to exit TUI: {}", e);
            return ExitCode::FAILURE;
        }
    }
    let _ = tx.send(Command::Quit);

    emulator_handle.join().unwrap();

    match res {
        Ok(()) => return ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    }
}

fn emulator_thread(
    computer: Arc<Mutex<HackComputer>>,
    rx: Receiver<Command>,
    tick_interval: Duration,
) {
    let mut running = false;

    loop {
        if running {
            match rx.recv_timeout(tick_interval) {
                Ok(Command::Run) => {}
                Ok(Command::Step) => {
                    if let Err(e) = computer.lock().unwrap().tick(false) {
                        eprintln!("error: {e}");
                        running = false;
                    }
                }
                Ok(Command::Quit) => break,
                Err(RecvTimeoutError::Timeout) => {
                    if let Err(e) = computer.lock().unwrap().tick(false) {
                        eprintln!("error: {e}");
                        running = false;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        } else {
            // idle: block until a command arrives, no ticking at all
            match rx.recv() {
                Ok(Command::Run) => running = true,
                Ok(Command::Step) => {
                    if let Err(e) = computer.lock().unwrap().tick(false) {
                        eprintln!("error: {e}");
                        running = false;
                    }
                }
                Ok(Command::Quit) => break,
                Err(_) => break, // sender dropped
            }
        }
    }
}

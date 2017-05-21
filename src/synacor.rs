mod machine;

use std::io;
use std::io::{BufReader, BufWriter};
use std::io::Write;
use std::fs::File;
use std::env;
use std::error::Error;
use std::str::FromStr;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn usage(arg0: &str) -> ! {
    println_stderr!("usage: {} <save-state> [eighth register value]", arg0);
    std::process::exit(1);
}

fn load_state(machine: &mut machine::Machine,
              filename: &str) -> io::Result<()> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    machine.load_state(&mut reader)
}

fn save_state(machine: &machine::Machine) -> io::Result<()> {
    let f = File::create("synacor-save")?;
    let mut writer = BufWriter::new(f);

    machine.save_state(&mut writer)
}

fn main() {
    let mut machine = machine::Machine::new();

    let mut args = env::args();

    let arg0 = args.next().unwrap();

    let save_state_filename = match args.next() {
        Some(n) => n,
        None => usage(&arg0)
    };

    let reg8 = args.next();

    match load_state(&mut machine, &save_state_filename) {
        Err(e) => {
            println_stderr!("{}", e.description());
            std::process::exit(1);
        },
        Ok(_) => ()
    }

    match reg8 {
        Some(n) => {
            let res = if n.starts_with("0x") {
                u16::from_str_radix(&n[2..], 16)
            } else {
                u16::from_str(&n)
            };
            match res {
                Err(_) => usage(&arg0),
                Ok(n) => machine.set_register(7, n)
            }
        },
        None => ()
    };

    loop {
        match machine.step() {
            Ok(_) => (),
            Err(msg) => {
                match msg {
                    machine::MachineError::Halted => (),
                    _ => println_stderr!("{}", msg.description())
                }
                break;
            }
        }
    }

    match save_state(&machine) {
        Ok(_) => (),
        Err(e) => {
            println_stderr!("Error saving: {}", e.description())
        }
    }
}

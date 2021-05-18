mod machine;

use std::io;
use std::io::{BufReader, BufWriter};
use std::io::Read;
use std::fs::File;
use std::env;
use std::str::FromStr;
use std::char;

fn usage(arg0: &str) -> ! {
    eprintln!("usage: {} <save-state> [eighth register value]", arg0);
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

struct StdIo {
}

impl machine::CharIo for StdIo {
    fn input(&self) -> io::Result<u16> {
        let mut buf = [0 as u8; 1];
        match io::stdin().read(&mut buf) {
            Ok(n) => {
                if n > 0 {
                    Ok(buf[0] as u16)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"))
                }
            },
            Err(e) => Err(e)
        }
    }

    fn output(&self, value: u16) -> io::Result<()> {
        if let Some(c) = char::from_u32(value as u32) {
            print!("{}", c);
        }
        Ok(())
    }
}

fn main() {
    let char_io = StdIo { };
    let mut machine = machine::Machine::new(&char_io);

    let mut args = env::args();

    let arg0 = args.next().unwrap();

    let save_state_filename = match args.next() {
        Some(n) => n,
        None => usage(&arg0)
    };

    let reg8 = args.next();

    if let Err(e) = load_state(&mut machine, &save_state_filename) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    if let Some(n) = reg8 {
        let res = if n.starts_with("0x") {
            u16::from_str_radix(&n[2..], 16)
        } else {
            u16::from_str(&n)
        };
        match res {
            Err(_) => usage(&arg0),
            Ok(n) => machine.set_register(7, n)
        }
    }

    loop {
        if let Err(msg) = machine.step() {
            match msg {
                machine::MachineError::Halted => (),
                _ => eprintln!("{}", msg.description())
            }
            break;
        }
    }

    if let Err(e) = save_state(&machine) {
        eprintln!("Error saving: {}", e)
    }
}

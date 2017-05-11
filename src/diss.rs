use std::io::Write;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::env;
use std::error::Error;
use std::char;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

struct Opcode {
    name: &'static str,
    n_arguments: u16
}

const opcodes: [Opcode; 23] = [
    Opcode { name: "halt", n_arguments: 0 },
    Opcode { name: "set", n_arguments: 2 },
    Opcode { name: "push", n_arguments: 1 },
    Opcode { name: "pop", n_arguments: 1 },
    Opcode { name: "eq", n_arguments: 3 },
    Opcode { name: "gt", n_arguments: 3 },
    Opcode { name: "jmp", n_arguments: 1 },
    Opcode { name: "jt", n_arguments: 2 },
    Opcode { name: "jf", n_arguments: 2 },
    Opcode { name: "add", n_arguments: 3 },
    Opcode { name: "mult", n_arguments: 3 },
    Opcode { name: "mod", n_arguments: 3 },
    Opcode { name: "and", n_arguments: 3 },
    Opcode { name: "or", n_arguments: 3 },
    Opcode { name: "not", n_arguments: 2 },
    Opcode { name: "rmem", n_arguments: 2 },
    Opcode { name: "wmem", n_arguments: 2 },
    Opcode { name: "call", n_arguments: 1 },
    Opcode { name: "ret", n_arguments: 0 },
    Opcode { name: "out", n_arguments: 1 },
    Opcode { name: "in", n_arguments: 1 },
    Opcode { name: "noop", n_arguments: 0 },
    Opcode { name: "?", n_arguments: 0 },
];

fn fetch(f: &mut BufReader<File>) -> Result<Option<u16>, std::io::Error> {
    let mut buf = [0 as u8; 2];

    let n = f.read(&mut buf)?;

    if n < 2 {
        Ok(None)
    } else {
        Ok(Some((buf[0] as u16) | ((buf[1] as u16) << 8)))
    }
}

fn dump_instruction(addr: usize,
                    f: &mut BufReader<File>,
                    opcode: u16) -> Result<usize, std::io::Error> {
    let ops = &opcodes;
    let mut instruction = if opcode as usize >= opcodes.len() {
        &ops[opcodes.len() - 1]
    } else {
        &ops[opcode as usize]
    };
    let mut args = Vec::<u16>::new();

    for i in 0..instruction.n_arguments {
        match fetch(f)? {
            None => {
                instruction = &ops[opcodes.len() - 1];
                break
            },
            Some(n) => args.push(n)
        }
    }

    print!("{:04x} {:4}", addr, instruction.name);

    for arg in &args {
        print!(" {:04x}", arg);
    }

    println!("");

    Ok(args.len())
}

fn diss_program(filename: &str) -> Result<(), std::io::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut addr: usize = 0;

    loop {
        match fetch(&mut reader)? {
            Some(opcode) => {
                addr += 1;
                addr += dump_instruction(addr, &mut reader, opcode)?;
            },
            None => break
        }
    }

    Ok(())
}

fn main() {
    for arg in env::args().skip(1) {
        match diss_program(&arg) {
            Err(e) => {
                println_stderr!("{}", e.description());
                std::process::exit(1);
            },
            Ok(_) => ()
        }
    }
}

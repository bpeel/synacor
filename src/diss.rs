use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::env;
use std::char;
use std::str::FromStr;

struct Opcode {
    name: &'static str,
    n_arguments: u16
}

const OPCODES: [Opcode; 23] = [
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
    let ops = &OPCODES;
    let mut instruction = if opcode as usize >= OPCODES.len() {
        &ops[OPCODES.len() - 1]
    } else {
        &ops[opcode as usize]
    };
    let mut args = Vec::<u16>::new();

    for _ in 0..instruction.n_arguments {
        match fetch(f)? {
            None => {
                instruction = &ops[OPCODES.len() - 1];
                break
            },
            Some(n) => args.push(n)
        }
    }

    print!("{:04x} {:4}", addr, instruction.name);

    for arg in &args {
        print!(" {:04x}", arg);
    }

    if opcode == 19 && args.len() > 0 {
        match char::from_u32(args[0] as u32) {
            Some(n) if n >= ' ' && n < '\u{80}' => print!(" // {}", n),
            _ => ()
        }
    }

    println!("");

    Ok(args.len())
}

fn diss_program(filename: &str, start_address: usize) -> Result<(), std::io::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut addr = start_address;

    for _ in 0..start_address {
        match fetch(&mut reader)? {
            None => return Ok(()),
            Some(_) => ()
        }
    }

    loop {
        match fetch(&mut reader)? {
            Some(opcode) => {
                addr += dump_instruction(addr, &mut reader, opcode)? + 1;
            },
            None => break
        }
    }

    Ok(())
}

fn usage(arg0: &str) -> ! {
    eprintln!("usage: {} <program> [start_address]", arg0);
    std::process::exit(1);
}

fn main() {
    let mut args = env::args();

    let arg0 = args.next().unwrap();

    let filename = match args.next() {
        Some(n) => n,
        None => usage(&arg0)
    };

    let start_address = match args.next() {
        Some(n) => {
            let res = if n.starts_with("0x") {
                usize::from_str_radix(&n[2..], 16)
            } else {
                usize::from_str(&n)
            };
            match res {
                Err(_) => usage(&arg0),
                Ok(n) => n
            }
        },
        None => 0
    };

    match diss_program(&filename, start_address) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(_) => ()
    }
}

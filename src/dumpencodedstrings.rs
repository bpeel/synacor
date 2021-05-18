use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::env;
use std::char;

const MEMORY_SIZE: usize = 0x7fff;

fn read_memory(memory: &mut[u16],
               length: usize,
               f: &mut BufReader<File>) -> Result<bool, std::io::Error> {
    let mut buf = [0 as u8; 2];
    let mut pos: usize = 0;

    while pos < length {
        let n = f.read(&mut buf)?;

        if n < 2 {
            return Ok(false)
        }

        memory[pos] = (buf[0] as u16) | ((buf[1] as u16) << 8);
        pos += 1;
    }

    Ok(true)
}

fn usage(arg0: &str) -> ! {
    eprintln!("usage: {} <save-state>", arg0);
    std::process::exit(1);
}


fn read_program(memory: &mut [u16],
                filename: &str) -> Result<(), std::io::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    read_memory(memory, MEMORY_SIZE, &mut reader)?;

    Ok(())
}

fn dump_strings(memory: &[u16]) {
    for i in 0..(MEMORY_SIZE - 12) {
        if memory[i] != 0x0001 ||
            memory[i + 1] != 0x8000 ||
            memory[i + 3] != 0x0001 ||
            memory[i + 4] != 0x8001 ||
            memory[i + 5] != 0x05fb ||
            memory[i + 6] != 0x0009 ||
            memory[i + 7] != 0x8002 ||
            memory[i + 10] != 0x0011 ||
            memory[i + 11] != 0x05b2 {
                continue
            }
        let arg_a = memory[i + 8];
        let arg_b = memory[i + 9];
        let mask = ((arg_a as u32 + arg_b as u32) & 0x7fff) as u16;
        let addr = memory[i + 2];

        if addr as usize >= MEMORY_SIZE {
            continue
        }

        let length = memory[addr as usize];

        if (length + addr + 1) as usize > MEMORY_SIZE {
            continue
        }

        print!("0x{:04x} ", addr);

        for i in addr + 1 .. addr + 1 + length {
            print!("{}",
                   match char::from_u32((memory[i as usize] ^ mask) as u32) {
                       Some(n) => n,
                       _ => '?'
                   });
        }
    }
}

fn main() {
    let mut memory = [0 as u16; MEMORY_SIZE];

    let mut args = env::args();

    let arg0 = args.next().unwrap();

    let save_state_filename = match args.next() {
        Some(n) => n,
        None => usage(&arg0)
    };

    if let Err(e) = read_program(&mut memory, &save_state_filename) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    dump_strings(&memory);
}

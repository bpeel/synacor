use std::char;

const OUT_OPCODE: u16 = 19;

fn fetch(f: &mut dyn std::io::Read) -> Option<u16> {
    let mut buf = [0 as u8; 2];

    match f.read(&mut buf) {
        Ok(n) if n >= 2 => Some((buf[0] as u16) | ((buf[1] as u16) << 8)),
        Ok(_) => None,
        Err(_) => None
    }
}

fn dump_string(start_address: usize, buf: &mut String) {
    if buf.len() > 0 {
        println!("0x{:04x}: {}", start_address, buf);
        buf.truncate(0);
    }
}

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut in_out = false;
    let mut start_address = 0;
    let mut addr = 0;

    loop {
        match fetch(&mut input) {
            Some(n) if n == OUT_OPCODE => {
                if in_out {
                    dump_string(start_address, &mut buf);
                }
                in_out = true;
                start_address = addr - 2;
            },
            Some(n) => {
                if in_out {
                    match char::from_u32(n as u32) {
                        Some(n) => buf.push(n),
                        None => dump_string(start_address, &mut buf)
                    }
                    in_out = false;
                } else {
                    dump_string(start_address, &mut buf);
                }
            },
            None => break
        }

        addr += 1;
    }
}

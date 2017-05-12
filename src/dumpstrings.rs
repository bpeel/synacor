use std::char;

const OUT_OPCODE: u16 = 19;

fn fetch(f: &mut std::io::Read) -> Option<u16> {
    let mut buf = [0 as u8; 2];

    match f.read(&mut buf) {
        Ok(n) if n >= 2 => Some((buf[0] as u16) | ((buf[1] as u16) << 8)),
        Ok(_) => None,
        Err(_) => None
    }
}

fn dump_string(buf: &mut String) {
    print!("{}", buf);
    buf.truncate(0);
}

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut in_out = false;

    loop {
        match fetch(&mut input) {
            Some(n) if n == OUT_OPCODE => {
                if in_out {
                    dump_string(&mut buf);
                }
                in_out = true;
            },
            Some(n) => {
                if in_out {
                    match char::from_u32(n as u32) {
                        Some(n) => buf.push(n),
                        None => dump_string(&mut buf)
                    }
                    in_out = false;
                } else {
                    dump_string(&mut buf);
                }
            },
            None => break
        }
    }
}

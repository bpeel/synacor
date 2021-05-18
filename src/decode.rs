use std::io::Write;
use std::io::Read;

fn fetch(f: &mut std::io::Stdin) -> Result<Option<u16>, std::io::Error> {
    let mut buf = [0 as u8; 2];

    let n = f.read(&mut buf)?;

    if n < 2 {
        Ok(None)
    } else {
        Ok(Some((buf[0] as u16) | ((buf[1] as u16) << 8)))
    }
}

fn decrypt(input: &mut std::io::Stdin,
           output: &mut std::io::Stdout) -> Result<(), std::io::Error> {
    let mut addr: u16 = 0;

    while let Some(val) = fetch(input)? {
        let square_address = ((addr as u32 * addr as u32) &
                              0x7fff) as u16;
        let decrypted = val ^ square_address ^ 0x4154;
        let buf: [u8; 2] = [
            decrypted as u8,
            (decrypted >> 8) as u8
        ];
        output.write(&buf)?;
        addr += 1;
    }

    Ok(())
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    if let Err(e) = decrypt(&mut stdin, &mut stdout) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

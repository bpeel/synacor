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

fn get_evens(input: &mut std::io::Stdin,
            output: &mut std::io::Stdout) -> Result<(), std::io::Error> {
    while let Some(val) = fetch(input)? {
        let buf = [ val as u8 ];
        output.write(&buf)?;
    }

    Ok(())
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    if let Err(e) = get_evens(&mut stdin, &mut stdout) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

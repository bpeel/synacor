use std::io::Write;
use std::io::Read;
use std::error::Error;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

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
    loop {
        match fetch(input)? {
            Some(val) => {
                let buf = [ val as u8 ];
                output.write(&buf)?;
            },
            None => break
        }
    }

    Ok(())
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    match get_evens(&mut stdin, &mut stdout) {
        Err(e) => {
            println_stderr!("{}", e.description());
            std::process::exit(1);
        },
        Ok(_) => ()
    }
}

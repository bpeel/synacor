use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::io::Read;
use std::fs::File;
use std::env;
use std::error::Error;
use std::char;
use std::str::FromStr;

const MEMORY_SIZE: usize = 0x7fff;
const N_REGISTERS: usize = 8;

enum MachineError {
    Halted,
    InvalidAddress,
    InvalidRegister,
    UnexpectedOpcode,
    StackUnderflow,
    InputError
}

impl MachineError {
    fn description(&self) -> &str {
        match *self {
            MachineError::Halted => "Halted",
            MachineError::InvalidAddress => "Invalid address",
            MachineError::InvalidRegister => "Invalid register",
            MachineError::UnexpectedOpcode => "Unexpected opcode",
            MachineError::StackUnderflow => "Stack underflow",
            MachineError::InputError => "Input error"
        }
    }
}

struct Machine {
    memory: [u16; MEMORY_SIZE],
    registers: [u16; N_REGISTERS],
    stack: Vec<u16>,
    pc: u16,
}

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

impl Machine {
    fn new() -> Machine {
        Machine {
            memory: [0; MEMORY_SIZE],
            registers: [0; N_REGISTERS],
            stack: Vec::<u16>::new(),
            pc: 0,
        }
    }

    fn read_memory(&self, address: usize) -> Result<u16, MachineError> {
        if address >= MEMORY_SIZE {
            Err(MachineError::InvalidAddress)
        } else {
            Ok(self.memory[address])
        }
    }

    fn write_memory(&mut self,
                    address: usize,
                    value: u16) -> Result<(), MachineError> {
        if address >= MEMORY_SIZE {
            Err(MachineError::InvalidAddress)
        } else {
            self.memory[address] = value;
            Ok(())
        }
    }

    fn fetch(&mut self) -> Result<u16, MachineError> {
        let res = self.read_memory(self.pc as usize);
        self.pc += 1;
        res
    }

    fn get_argument(&self, value: u16) -> Result<u16, MachineError> {
        if value < 0x8000 {
            Ok(value)
        } else {
            let register = value - 0x8000;
            if register as usize >= N_REGISTERS {
                Err(MachineError::InvalidRegister)
            } else {
                Ok(self.registers[register as usize])
            }
        }
    }

    fn fetch_argument(&mut self) -> Result<u16, MachineError> {
        let arg = self.fetch()?;
        self.get_argument(arg)
    }

    fn fetch_register(&mut self) -> Result<usize, MachineError> {
        let arg = self.fetch()?;
        Machine::check_register(arg)
    }

    fn check_register(value: u16) -> Result<usize, MachineError> {
        if value < 0x8000 || value as usize >= 0x8000 + N_REGISTERS {
            Err(MachineError::InvalidRegister)
        } else {
            Ok((value - 0x8000) as usize)
        }
    }

    fn step(&mut self) -> Result<(), MachineError> {
        let opcode = self.fetch()?;

        macro_rules! arithmetic_op {
            ($b:ident, $c:ident, $op:expr) => {{
                let register = self.fetch_register()?;
                let $b = self.fetch_argument()?;
                let $c = self.fetch_argument()?;
                self.registers[register] = $op;
                Ok(())
            }}
        }

        match opcode {
            0 => Err(MachineError::Halted),
            1 => {
                let register = self.fetch_register()?;
                self.registers[register] = self.fetch_argument()?;
                Ok(())
            },
            2 => {
                let value = self.fetch_argument()?;
                self.stack.push(value);
                Ok(())
            },
            3 => {
                match self.stack.pop() {
                    None => Err(MachineError::StackUnderflow),
                    Some(v) => {
                        let register = self.fetch_register()?;
                        self.registers[register] = v;
                        Ok(())
                    }
                }
            },
            4 => arithmetic_op!(b, c, if b == c { 1 } else { 0 }),
            5 => arithmetic_op!(b, c, if b > c { 1 } else { 0 }),
            6 => {
                self.pc = self.fetch_argument()?;
                Ok(())
            },
            7 => {
                let a = self.fetch_argument()?;
                let b = self.fetch_argument()?;
                if a != 0 {
                    self.pc = b;
                }
                Ok(())
            },
            8 => {
                let a = self.fetch_argument()?;
                let b = self.fetch_argument()?;
                if a == 0 {
                    self.pc = b;
                }
                Ok(())
            },
            9 => arithmetic_op!(b, c, (b + c) & 0x7fff),
            10 => arithmetic_op!(b, c, ((b as u32 * c as u32) & 0x7fff) as u16),
            11 => arithmetic_op!(b, c, b % c),
            12 => arithmetic_op!(b, c, b & c),
            13 => arithmetic_op!(b, c, b | c),
            14 => {
                let register = self.fetch_register()?;
                let b = self.fetch_argument()?;
                self.registers[register] = b ^ 0x7fff;
                Ok(())
            },
            15 => {
                let register = self.fetch_register()?;
                let b = self.fetch_argument()?;
                self.registers[register] = self.read_memory(b as usize)?;
                Ok(())
            },
            16 => {
                let a = self.fetch_argument()?;
                let b = self.fetch_argument()?;
                self.write_memory(a as usize, b)
            },
            17 => {
                let a = self.fetch_argument()?;
                self.stack.push(self.pc);
                self.pc = a;
                Ok(())
            },
            18 => {
                match self.stack.pop() {
                    None => Err(MachineError::Halted),
                    Some(v) => {
                        self.pc = v;
                        Ok(())
                    }
                }
            },
            19 => {
                match char::from_u32(self.fetch_argument()? as u32) {
                    Some(c) => print!("{}", c),
                    None => ()
                };
                Ok(())
            },
            20 => {
                let register = self.fetch_register()?;
                let mut buf = [0 as u8; 1];
                match std::io::stdin().read(&mut buf) {
                    Ok(n) if n > 0 => {
                        self.registers[register] = buf[0] as u16;
                        Ok(())
                    },
                    _ => {
                        /* Put the pc back to the read instruction so
                         * that it will continue from there if the
                         * machine is reused */
                        self.pc -= 2;
                        Err(MachineError::InputError)
                    }
                }
            },
            21 => Ok(()), /* nop */
            _ => {
                println_stderr!("opcode {}", opcode);
                Err(MachineError::UnexpectedOpcode)
            }
        }
    }
}

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

fn read_program(machine: &mut Machine,
                filename: &str) -> Result<(), std::io::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    if !read_memory(&mut machine.memory, MEMORY_SIZE, &mut reader)? {
        return Ok(())
    }

    if !read_memory(&mut machine.registers, N_REGISTERS, &mut reader)? {
        return Ok(())
    }

    let mut buf = [0 as u8; 2];
    let n = reader.read(&mut buf)?;

    if n < 2 {
        return Ok(())
    }

    machine.pc = (buf[0] as u16) | ((buf[1] as u16) << 8);

    loop {
        let n = reader.read(&mut buf)?;

        if n < 2 {
            return Ok(())
        }

        machine.stack.push((buf[0] as u16) | ((buf[1] as u16) << 8));
    }
}


fn save_memory(memory: &[u16],
               length: usize,
               f: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    let mut buf = [0 as u8; 2];

    for pos in 0..length {
        buf[0] = memory[pos] as u8;
        buf[1] = (memory[pos] >> 8) as u8;
        f.write(&buf)?;
    }

    Ok(())
}

fn save_program(machine: &Machine) -> Result<(), std::io::Error> {
    let f = File::create("synacor-save")?;
    let mut writer = BufWriter::new(f);

    save_memory(&machine.memory, MEMORY_SIZE, &mut writer)?;
    save_memory(&machine.registers, N_REGISTERS, &mut writer)?;

    let buf = [
        machine.pc as u8,
        (machine.pc >> 8) as u8
    ];

    writer.write(&buf)?;

    for val in &machine.stack {
        let buf = [
            *val as u8,
            (*val >> 8) as u8
        ];
        writer.write(&buf)?;
    }

    Ok(())
}

fn usage(arg0: &str) -> ! {
    println_stderr!("usage: {} <save-state> [eighth register value]", arg0);
    std::process::exit(1);
}

fn main() {
    let mut machine = Machine::new();

    let mut args = env::args();

    let arg0 = args.next().unwrap();

    let save_state_filename = match args.next() {
        Some(n) => n,
        None => usage(&arg0)
    };

    let reg8 = args.next();

    match read_program(&mut machine, &save_state_filename) {
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
                Ok(n) => machine.registers[7] = n
            }
        },
        None => ()
    };

    loop {
        match machine.step() {
            Ok(_) => (),
            Err(msg) => {
                match msg {
                    MachineError::Halted => (),
                    _ => println_stderr!("{}", msg.description())
                }
                break;
            }
        }
    }

    match save_program(&machine) {
            Ok(_) => (),
            Err(e) => {
                println_stderr!("Error saving: {}", e.description())
            }
    }
}

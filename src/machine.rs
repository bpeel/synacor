use std::io;
use std::io::{Read, Write};

pub const MEMORY_SIZE: usize = 0x7fff;
pub const N_REGISTERS: usize = 8;

pub enum MachineError {
    Halted,
    InvalidAddress,
    InvalidRegister,
    UnexpectedOpcode,
    StackUnderflow,
    InputError
}

impl MachineError {
    pub fn description(&self) -> &str {
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

pub trait CharIo {
    fn input(&self) -> io::Result<u16>;
    fn output(&self, value: u16) -> io::Result<()>;
}

pub struct Machine<'a> {
    memory: [u16; MEMORY_SIZE],
    registers: [u16; N_REGISTERS],
    stack: Vec<u16>,
    pc: u16,
    char_io: &'a dyn CharIo
}

impl<'a> Machine<'a> {
    pub fn new(char_io: &'a dyn CharIo) -> Machine {
        Machine {
            memory: [0; MEMORY_SIZE],
            registers: [0; N_REGISTERS],
            stack: Vec::<u16>::new(),
            pc: 0,
            char_io: char_io
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

    pub fn step(&mut self) -> Result<(), MachineError> {
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
                let value = self.fetch_argument()?;
                if let Err(_) = self.char_io.output(value) {
                    Err(MachineError::InputError)
                } else {
                    Ok(())
                }
            },
            20 => {
                let register = self.fetch_register()?;
                match self.char_io.input() {
                    Ok(value) => {
                        self.registers[register] = value;
                        Ok(())
                    },
                    _=> {
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
                Err(MachineError::UnexpectedOpcode)
            }
        }
    }

    fn load_memory(memory: &mut[u16],
                   length: usize,
                   f: &mut dyn Read) -> io::Result<bool> {
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

    pub fn load_state(&mut self,
                      reader: &mut dyn Read) -> io::Result<()> {
        if !Machine::load_memory(&mut self.memory, MEMORY_SIZE, reader)? {
            return Ok(())
        }

        if !Machine::load_memory(&mut self.registers, N_REGISTERS, reader)? {
            return Ok(())
        }

        let mut buf = [0 as u8; 2];
        let n = reader.read(&mut buf)?;

        if n < 2 {
            return Ok(())
        }

        self.pc = (buf[0] as u16) | ((buf[1] as u16) << 8);
        self.stack.truncate(0);

        loop {
            let n = reader.read(&mut buf)?;

            if n < 2 {
                return Ok(())
            }

            self.stack.push((buf[0] as u16) | ((buf[1] as u16) << 8));
        }
    }

    fn save_memory(memory: &[u16],
                   length: usize,
                   f: &mut dyn Write) -> io::Result<()> {
        let mut buf = [0 as u8; 2];

        for pos in 0..length {
            buf[0] = memory[pos] as u8;
            buf[1] = (memory[pos] >> 8) as u8;
            f.write(&buf)?;
        }

        Ok(())
    }

    pub fn save_state(&self,
                      writer: &mut dyn Write) -> io::Result<()> {
        Machine::save_memory(&self.memory, MEMORY_SIZE, writer)?;
        Machine::save_memory(&self.registers, N_REGISTERS, writer)?;

        let buf = [
            self.pc as u8,
            (self.pc >> 8) as u8
        ];

        writer.write(&buf)?;

        for val in &self.stack {
            let buf = [
                *val as u8,
                (*val >> 8) as u8
            ];
            writer.write(&buf)?;
        }

        Ok(())
    }

    pub fn set_register(&mut self,
                        register_num: usize,
                        value: u16) {
        self.registers[register_num] = value;
    }
}

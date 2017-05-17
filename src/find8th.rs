fn minus_one(a: u16) -> u16 {
    ((a as u32 + 0x7fff) & 0x7fff) as u16
}

fn thing1(eighth: u16, a: u16, b: u16) -> u16 {
    if a != 0 {
        thing2(eighth, a, b)
    } else {
        b + 1
    }
}

fn thing2(eighth: u16, a: u16, b: u16) -> u16 {
    if b != 0 {
        thing3(eighth, a, b)
    } else {
        thing1(eighth, minus_one(a), eighth)
    }
}

fn thing3(eighth: u16, a: u16, b: u16) -> u16 {
    thing1(eighth, minus_one(a), thing1(eighth, a, minus_one(b)))
}

fn main() {
    for eighth in 0..0x8000 {
        println!("{:04x} {:04x}", eighth, thing1(eighth, 4, 1))
    }
}

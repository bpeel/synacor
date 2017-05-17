fn minus_one(a: u16) -> u16 {
    ((a as u32 + 0x7fff) & 0x7fff) as u16
}

struct Finder {
    eighth: u16
}

impl Finder {
    fn new(eighth: u16) -> Finder {
        Finder {
            eighth: eighth
        }
    }

    fn thing1(&self, a: u16, b: u16) -> u16 {
        if a != 0 {
            self.thing2(a, b)
        } else {
            b + 1
        }
    }

    fn thing2(&self, a: u16, b: u16) -> u16 {
        if b != 0 {
            self.thing3(a, b)
        } else {
            self.thing1(minus_one(a), self.eighth)
        }
    }

    fn thing3(&self, a: u16, b: u16) -> u16 {
        self.thing1(minus_one(a), self.thing1(a, minus_one(b)))
    }
}

fn main() {
    for eighth in 0..0x8000 {
        let finder = Finder::new(eighth);
        println!("{:04x} {:04x}", eighth, finder.thing1(4, 1))
    }
}

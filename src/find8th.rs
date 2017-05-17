use std::collections::HashMap;

fn minus_one(a: u16) -> u16 {
    ((a as u32 + 0x7fff) & 0x7fff) as u16
}

struct Finder {
    eighth: u16,
    cache1: HashMap<(u16, u16), u16>,
    cache2: HashMap<(u16, u16), u16>,
    cache3: HashMap<(u16, u16), u16>
}

impl Finder {
    fn new(eighth: u16) -> Finder {
        Finder {
            eighth: eighth,
            cache1: HashMap::new(),
            cache2: HashMap::new(),
            cache3: HashMap::new()
        }
    }

    fn thing1(&mut self, a: u16, b: u16) -> u16 {
        let key = (a, b);

        match self.cache1.get(&key).cloned() {
            Some(n) => n,
            None => {
                let v = if a != 0 {
                    self.thing2(a, b)
                } else {
                    b + 1
                };
                self.cache1.insert(key, v);
                v
            }
        }
    }

    fn thing2(&mut self, a: u16, b: u16) -> u16 {
        let key = (a, b);

        match self.cache2.get(&key).cloned() {
            Some(n) => n,
            None => {
                let v = if b != 0 {
                    self.thing3(a, b)
                } else {
                    let b = self.eighth;
                    self.thing1(minus_one(a), b)
                };
                self.cache2.insert(key, v);
                v
            }
        }
    }

    fn thing3(&mut self, a: u16, b: u16) -> u16 {
        let key = (a, b);

        match self.cache3.get(&key).cloned() {
            Some(n) => n,
            None => {
                let v = {
                    let b = self.thing1(a, minus_one(b));
                    self.thing1(minus_one(a), b)
                };
                self.cache3.insert(key, v);
                v
            }
        }
    }
}

fn main() {
    for eighth in 0..0x8000 {
        let mut finder = Finder::new(eighth);
        println!("{:04x} {:04x}", eighth, finder.thing1(4, 1))
    }
}

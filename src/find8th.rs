use std::collections::HashMap;
use std::thread;

const N_THREADS: u16 = 4;

fn minus_one(a: u16) -> u16 {
    ((a as u32 + 0x7fff) & 0x7fff) as u16
}

struct Finder {
    eighth: u16,
    cache: HashMap<(u16, u16), u16>,
}

impl Finder {
    fn new(eighth: u16) -> Finder {
        Finder {
            eighth: eighth,
            cache: HashMap::new()
        }
    }

    fn thing1(&mut self, a: u16, b: u16) -> u16 {
        let key = (a, b);

        match self.cache.get(&key).cloned() {
            Some(n) => n,
            None => {
                let v = if a != 0 {
                    self.thing2(a, b)
                } else {
                    b + 1
                };
                self.cache.insert(key, v);
                v
            }
        }
    }

    fn thing2(&mut self, a: u16, b: u16) -> u16 {
        if b != 0 {
            self.thing3(a, b)
        } else {
            let b = self.eighth;
            self.thing1(minus_one(a), b)
        }
    }

    fn thing3(&mut self, a: u16, b: u16) -> u16 {
        let b = self.thing1(a, minus_one(b));
        self.thing1(minus_one(a), b)
    }
}

fn find_solution(offset: u16) {
    let mut eighth = offset;

    while eighth <= 0x7fff {
        let mut finder = Finder::new(eighth);

        let n = finder.thing1(4, 1);
        if n == 6 {
            println!("{:04x} {:04x}", eighth, n);
        }

        eighth += N_THREADS;
    }
}

fn main() {
    let mut threads = Vec::<thread::JoinHandle<()>>::new();

    for offset in 0..N_THREADS {
        let builder = thread::Builder::new().stack_size(512 * 1024 * 1024);
        threads.push(builder.spawn(move || {
            find_solution(offset);
        }).unwrap());
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

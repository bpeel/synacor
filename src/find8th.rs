use std::collections::HashMap;
use std::thread;

const N_THREADS: u16 = 4;

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

    fn compute_uncached(&mut self, a: u16, b: u16) -> u16 {
        if a != 0 {
            if b != 0 {
                let b = self.compute(a, b - 1);
                self.compute(a - 1, b)
            } else {
                let b = self.eighth;
                self.compute(a - 1, b)
            }
        } else {
            b + 1
        }
    }

    fn compute(&mut self, a: u16, b: u16) -> u16 {
        let key = (a, b);

        match self.cache.get(&key).cloned() {
            Some(n) => n,
            None => {
                let v = self.compute_uncached(a, b);
                self.cache.insert(key, v);
                v
            }
        }
    }
}

fn find_solution(offset: u16) {
    let mut eighth = offset;

    while eighth <= 0x7fff {
        let mut finder = Finder::new(eighth);

        let n = finder.compute(4, 1);
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

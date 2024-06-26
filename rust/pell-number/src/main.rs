use once_cell::sync::Lazy;

static mut PELL_VALUES: Lazy<Vec<u128>> = Lazy::new(|| vec![0, 1]);

fn pell_mut_static(n: u32) -> u128 {
    unsafe { PELL_VALUES.get(n as usize) }
        .copied()
        .unwrap_or_else(|| {
            let val: u128 = 2 * pell_mut_static(n - 1) + pell_mut_static(n - 2);
            unsafe { PELL_VALUES.push(val) }
            val
        })
}

fn pell_recurse(n: u32) -> u128 {
    if n <= 1 {
        return n as u128;
    }
    pell_recurse(n - 2) + 2 * pell_recurse(n - 1)
}

struct PellGenerator {
    values: Vec<u128>,
}

impl PellGenerator {
    fn new() -> Self {
        Self { values: vec![0, 1] }
    }

    fn get(&mut self, n: u32) -> u128 {
        self.values.get(n as usize).copied().unwrap_or_else(|| {
            let val = self.get(n - 2) + 2 * self.get(n - 1);
            self.values.push(val);
            val
        })
    }
}

// Realistically we should not have to create a new generator every time
fn pell_generator(n: u32) -> u128 {
    let mut gen = PellGenerator::new();
    gen.get(n)
}

fn pell_fib(n: u32) -> u128 {
    if n < 2 {
        return n as u128;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..(n - 1) {
        c = 2 * b + a;
        a = b;
        b = c;
    }
    c
}

macro_rules! run_pell {
    ($($x:tt),+) => {
        $(
            for i in 0..10 {
                println!("{}", $x(i));
            }
        );+
    }
}

fn main() {
    run_pell!(pell_mut_static, pell_recurse, pell_generator, pell_fib);
}

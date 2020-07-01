use cargo_snippet::snippet;

#[snippet("inv")]
fn inv(a: usize, m: usize) -> usize {
    let m = m as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

#[snippet("Combination")]
pub struct Combination {
    m: usize,
    f: Vec<usize>,
    g: Vec<usize>,
}

#[snippet("Combination")]
impl Combination {
    pub fn new(m: usize) -> Combination {
        Combination {
            m,
            f: vec![1],
            g: vec![1],
        }
    }

    pub fn combinations(&mut self, n: usize, k: usize) -> usize {
        for i in self.f.len()..=n {
            self.f.push(self.f[i - 1] * i % self.m);
            self.g.push(inv(self.f[i], self.m));
        }
        self.f[n] * self.g[k] % self.m * self.g[n - k] % self.m
    }
}

#[snippet("modpow")]
pub fn modpow(x: usize, y: usize, m: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % m;
        }
        a = a * a % m;
        b /= 2;
    }
    result
}

use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: u64 = 998244353;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars,
    }
    let mut a = a
        .into_iter()
        .map(|ai| Complex64::new((ai as u8 - '0' as u8) as f64, 0.))
        .collect::<Vec<_>>();
    let mut b = b
        .into_iter()
        .map(|bi| Complex64::new((bi as u8 - '0' as u8) as f64, 0.))
        .collect::<Vec<_>>();
    a.reverse();
    b.reverse();
    for i in 0..n {
        if a[i].re < b[i].re {
            let tmp = a[i];
            a[i] = b[i];
            b[i] = tmp;
        }
    }
    let c = convolution_fast(&a, &b);
    let mut base = 1;
    let mut result = 0;
    for i in 0..n * 2 - 1 {
        result = (result + base * c[i].re.round() as u64) % M;
        base = (base * 10) % M;
    }
    println!("{}", result);
}

use num::complex::Complex64;
use std::f64::consts::PI;
fn ceil_log(n: usize) -> usize {
    let mut ret = 0;
    let mut x = 1;
    while x < n {
        x <<= 1;
        ret += 1;
    }
    ret
}
fn bit_reversal(n: usize) -> Vec<usize> {
    let mut ret = vec![0; n];
    let log = ceil_log(n);
    for d in 0..log {
        for i in 0..1 << d {
            ret[i | (1 << d)] = ret[i] | (1 << (log - 1 - d));
        }
    }
    ret
}
pub fn dft_fast(f: &[Complex64]) -> Vec<Complex64> {
    let log = ceil_log(f.len());
    let n = 1 << log;
    let r = bit_reversal(1 << log);
    let mut ret = (0..n)
        .map(|k| *f.get(r[k]).unwrap_or(&Complex64::new(0., 0.)))
        .collect::<Vec<_>>();
    for d in 0..log {
        let w = 1 << d;
        let o = Complex64::from_polar(&1., &(PI * 2. / (w << 1) as f64));
        for l in (0..(1 << log)).step_by(w << 1) {
            for k in 0..w {
                let o = o.powu(k as u32);
                /* (ret[l + k], ret[l + k + w]) = (
                    ret[l + k] + o * ret[l + k + w],
                    ret[l + k] - o * ret[l + k + w],
                ); */
                let z1 = ret[l + k] + o * ret[l + k + w];
                let z2 = ret[l + k] - o * ret[l + k + w];
                ret[l + k] = z1;
                ret[l + k + w] = z2;
            }
        }
    }
    ret
}
pub fn idft_fast(f: &Vec<Complex64>) -> Vec<Complex64> {
    let n = f.len();
    let f = dft_fast(f);
    (0..n)
        .map(|k| {
            if k == 0 {
                f[k] / n as f64
            } else {
                f[n - k] / n as f64
            }
        })
        .collect::<Vec<_>>()
}
pub fn convolution_fast(f: &[Complex64], g: &[Complex64]) -> Vec<Complex64> {
    let m1 = 1 << ceil_log(f.len());
    let m2 = 1 << ceil_log(g.len());
    let n = m1 + m2;
    let mut f = Vec::from(f);
    let mut g = Vec::from(g);
    f.resize(n, Complex64::new(0., 0.));
    g.resize(n, Complex64::new(0., 0.));

    let f = dft_fast(&f);
    let g = dft_fast(&g);
    let fg = (0..n).map(|i| f[i] * g[i]).collect::<Vec<_>>();
    idft_fast(&fg)
}

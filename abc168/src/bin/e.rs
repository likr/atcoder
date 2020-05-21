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
const M: usize = 1000000007;

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        gcd(b, a)
    } else if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn modpow(x: usize, y: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}

fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n],
    }
    let mut factors: HashMap<(usize, usize, bool), usize> = HashMap::new();
    let mut zero_zero = 0usize;
    for i in 0..n {
        let (ai, bi) = ab[i];
        if ai == 0 && bi == 0 {
            zero_zero += 1;
        } else if ai == 0 {
            *factors.entry((0, 1, true)).or_insert(0) += 1;
        } else if bi == 0 {
            *factors.entry((1, 0, false)).or_insert(0) += 1;
        } else if (ai > 0 && bi > 0) || (ai < 0 && bi < 0) {
            let ai = ai.abs() as usize;
            let bi = bi.abs() as usize;
            let c = gcd(ai, bi);
            *factors.entry((ai / c, bi / c, true)).or_insert(0) += 1;
        } else {
            let ai = ai.abs() as usize;
            let bi = bi.abs() as usize;
            let c = gcd(ai, bi);
            *factors.entry((ai / c, bi / c, false)).or_insert(0) += 1;
        }
    }
    // eprintln!("{:?}", factors);
    let mut result = 1;
    for (&key, &c1) in factors.iter() {
        let (ai, bi, positive) = key;
        if let Some(&c2) = factors.get(&(bi, ai, !positive)) {
            if positive {
                let x = modpow(2, c1);
                let y = modpow(2, c2);
                let z = (x + y + M - 1) % M;
                result = (result * z) % M;
            }
        } else {
            result = result * modpow(2, c1) % M;
        }
    }
    result = (result + zero_zero) % M;
    result = (result + M - 1) % M;
    println!("{}", result);
}

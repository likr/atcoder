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

fn main() {
    input! {
        mut n: usize,
    }
    let mut prime_factors = vec![];
    let mut d = 2;
    while n > 1 {
        if d * d > n {
            break;
        }
        while n % d == 0 {
            prime_factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        prime_factors.push(n);
    }
    eprintln!("{:?}", prime_factors);

    let mut count = HashMap::new();
    for &f in &prime_factors {
        *count.entry(f).or_insert(0) += 1usize;
    }
    eprintln!("{:?}", count);

    let mut result = 0;
    for &c in count.values() {
        let mut s = 0;
        for j in 1.. {
            s += j;
            if s > c {
                result += j - 1;
                break;
            }
        }
    }
    println!("{}", result);
}

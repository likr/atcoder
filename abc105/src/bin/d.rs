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
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut acc = vec![0; n];
    acc[0] = a[0] % m;
    for i in 1..n {
        acc[i] = (acc[i - 1] + a[i]) % m;
    }
    // eprintln!("{:?}", acc);
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(acc[i]).or_insert(0) += 1;
    }
    // eprintln!("{:?}", count);
    let mut s = 0usize;
    if let Some(&v) = count.get(&0) {
        s += v;
    }
    for &v in count.values() {
        s += v * (v - 1) / 2;
    }

    println!("{}", s);
}

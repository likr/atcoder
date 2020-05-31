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
        mut t: [usize; n],
    }
    t.sort();

    let mut count = HashMap::new();
    for &ti in &t {
        *count.entry(ti).or_insert(0) += 1usize;
    }

    for i in 1..n {
        t[i] += t[i - 1];
    }
    let s = t.iter().sum::<usize>();

    let mut f = vec![1; n + 1];
    for i in 2..=n {
        f[i] = (i * f[i - 1]) % M;
    }

    let mut patterns = 1usize;
    for &c in count.values() {
        patterns = (patterns * f[c]) % M;
    }

    println!("{}", s);
    println!("{}", patterns);
}

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
        a: [isize; n],
    }
    let mut l = HashMap::new();
    let mut r = HashMap::new();
    for i in 0..n {
        *l.entry(i as isize + a[i]).or_insert(0) += 1isize;
        *r.entry(i as isize - a[i]).or_insert(0) += 1isize;
    }

    let mut result = 0;
    for (v, c) in l.iter() {
        if let Some(&d) = r.get(&v) {
            result += c * d;
        }
    }
    println!("{}", result);
}

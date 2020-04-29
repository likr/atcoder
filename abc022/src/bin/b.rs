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
        a: [usize; n],
    }
    let mut visited = HashSet::new();
    let mut count = 0;
    for i in 0..n {
        if visited.contains(&a[i]) {
            count += 1;
        }
        visited.insert(a[i]);
    }
    println!("{}", count);
}

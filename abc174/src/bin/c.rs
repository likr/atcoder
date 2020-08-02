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
        k: usize,
    }
    let mut x = 0;
    let mut i = 0;
    let mut visited = HashSet::new();
    loop {
        i += 1;
        x = x * 10 + 7;
        if visited.contains(&x) {
            println!("-1");
            return;
        }
        visited.insert(x);
        x %= k;
        if x == 0 {
            break;
        }
    }
    println!("{}", i);
}

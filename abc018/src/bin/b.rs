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
        mut s: Chars,
        n: usize,
        lr: [(Usize1, Usize1); n],
    }
    for i in 0..n {
        let (li, ri) = lr[i];
        let m = (ri - li + 1) / 2;
        for j in 0..m {
            eprintln!("{} {}", li + j, ri - j);
            s.swap(li + j, ri - j);
        }
    }
    println!("{}", s.iter().collect::<String>());
}

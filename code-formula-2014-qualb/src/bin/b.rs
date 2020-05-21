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
        n: Chars,
    }
    let mut n = n
        .iter()
        .map(|&c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    n.reverse();
    let m = n.len();
    let mut o = 0usize;
    let mut e = 0usize;
    for i in 0..m {
        if i % 2 == 0 {
            o += n[i];
        } else {
            e += n[i];
        }
    }
    println!("{} {}", e, o);
}

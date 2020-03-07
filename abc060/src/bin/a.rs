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
        a: Chars,
        b: Chars,
        c: Chars,
    }
    if a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0] {
        println!("YES");
    } else {
        println!("NO");
    }
}

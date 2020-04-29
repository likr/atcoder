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
        s: Chars,
    }
    let n = s.len();
    for l in (2..n).step_by(2) {
        let m = (n - l) / 2;
        if (0..m).all(|i| s[i] == s[i + m]) {
            println!("{}", m * 2);
            return;
        }
    }
}

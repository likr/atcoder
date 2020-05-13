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
    let keyence = "keyence".chars().collect::<Vec<_>>();
    let m = keyence.len();
    for k in 0..=m {
        if (0..k).all(|i| keyence[i] == s[i]) && (k..m).all(|i| keyence[i] == s[n + i - m]) {
            println!("YES");
            return;
        }
    }
    println!("NO");
}

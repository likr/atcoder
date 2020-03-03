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
    let t = "WBWBWBW".chars().collect::<Vec<_>>();
    let m = t.len();
    for i in 0..=n - m {
        if (0..m).all(|j| s[i + j] == t[j]) {
            println!(
                "{}",
                match i % 12 {
                    0 => "Fa",
                    1 => "Mi",
                    3 => "Re",
                    5 => "Do",
                    6 => "Si",
                    8 => "La",
                    10 => "So",
                    _ => "",
                }
            );
            return;
        }
    }
}

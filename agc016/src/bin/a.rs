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
    let s = s
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();

    let mut result = INF;
    for c in 0..26 {
        let mut t = s.clone();
        for k in (1..=n).rev() {
            if (0..k).all(|i| t[i] == c) {
                result = min(result, n - k);
                break;
            }
            for i in 1..k {
                if t[i] == c {
                    t[i - 1] = c;
                }
            }
        }
    }
    println!("{}", result);
}

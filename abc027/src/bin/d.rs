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
    let mut x = vec![];
    let mut plus_count = 0isize;
    let mut minus_count = 0isize;
    for i in (0..n).rev() {
        match s[i] {
            '+' => {
                plus_count += 1;
            }
            '-' => {
                minus_count += 1;
            }
            _ => {
                x.push(plus_count - minus_count);
            }
        }
    }
    x.sort();
    let m = x.len();
    let mut score = 0isize;
    for i in 0..m / 2 {
        score -= x[i];
    }
    for i in m / 2..m {
        score += x[i];
    }
    println!("{}", score);
}

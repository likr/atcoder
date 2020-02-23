use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
    let mut count = vec![0; 3];
    for i in 0..n {
        if s[i] == 'a' {
            count[0] += 1;
        } else if s[i] == 'b' {
            count[1] += 1;
        } else {
            count[2] += 1;
        }
    }
    let count_min = count.iter().min().unwrap();
    let count_max = count.iter().max().unwrap();
    if count_max - count_min >= 2 {
        println!("NO");
    } else {
        println!("YES");
    }
}

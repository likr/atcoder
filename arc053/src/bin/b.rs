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
    let mut count = vec![0usize; 26];
    for &c in &s {
        count[c as usize - 'a' as usize] += 1;
    }

    let mut odd_count = 0;
    for i in 0..26 {
        if count[i] % 2 == 1 {
            odd_count += 1;
        }
    }
    if odd_count <= 1 {
        println!("{}", n);
    } else {
        println!("{}", 1 + (n - odd_count) / 2 / odd_count * 2);
    }
}

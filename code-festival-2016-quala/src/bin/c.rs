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
        mut s: Chars,
        mut k: usize,
    }
    let n = s.len();
    for i in 0..n {
        let d = 'z' as usize - s[i] as usize + 1;
        if s[i] != 'a' && d <= k {
            s[i] = 'a';
            k -= d;
        }
    }
    // eprintln!("{}", k);
    let c = ((s[n - 1] as usize - 'a' as usize) + k) % 26;
    s[n - 1] = ('a' as usize + c) as u8 as char;
    for &c in &s {
        print!("{}", c);
    }
    println!();
}

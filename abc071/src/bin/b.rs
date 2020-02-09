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
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let mut t = HashSet::new();
    for &c in &s {
        t.insert(c);
    }
    for c in chars.chars() {
        if !t.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}

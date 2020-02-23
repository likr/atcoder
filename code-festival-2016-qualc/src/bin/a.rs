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
    if let Some(i) = s.iter().position(|&c| c == 'C') {
        if let Some(j) = s.iter().rposition(|&c| c == 'F') {
            if i < j {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

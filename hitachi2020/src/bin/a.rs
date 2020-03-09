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
    if s.len() % 2 == 0
        && (0..s.len())
            .step_by(2)
            .all(|i| s[i] == 'h' && s[i + 1] == 'i')
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

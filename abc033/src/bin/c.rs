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
        s: String,
    }
    let mut result = 0;
    for t in s.split('+') {
        let nums = t.split('*').collect::<HashSet<_>>();
        if !nums.contains(&"0") {
            result += 1;
        }
    }
    println!("{}", result);
}

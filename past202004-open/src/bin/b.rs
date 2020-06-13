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
    let mut count = HashMap::new();
    for &c in &s {
        *count.entry(c).or_insert(0) += 1;
    }
    println!(
        "{}",
        "abc"
            .chars()
            .max_by_key(|c| {
                if let Some(&v) = count.get(&c) {
                    v
                } else {
                    0
                }
            })
            .unwrap()
    );
}

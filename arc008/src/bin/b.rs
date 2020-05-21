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
        n: usize,
        _m: usize,
        name: Chars,
        kit: Chars,
    }
    let mut count = HashMap::new();
    for &c in &name {
        *count.entry(c).or_insert(0) += 1;
    }
    for i in 0..n {
        for &d in &kit {
            if let Some(c) = count.get_mut(&d) {
                if *c > 0 {
                    *c -= 1;
                }
            }
        }
        if count.values().all(|&c| c == 0) {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}

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
        k: usize,
        v: [isize; n],
    }
    let mut result = 0;
    for left in 0..=min(k, n) {
        for right in 0..=min(k - left, n) {
            if left + right > n {
                continue;
            }
            let mut items = vec![];
            for i in 0..left {
                items.push(v[i]);
            }
            for i in n - right..n {
                items.push(v[i]);
            }
            items.sort();
            let mut s = items.iter().sum::<isize>();
            let remove = k - left - right;
            // eprintln!("{} {} {}", left, right, remove);
            for i in 0..min(remove, items.len()) {
                if items[i] < 0 {
                    s -= items[i];
                }
            }
            result = max(result, s);
        }
    }
    println!("{}", result);
}

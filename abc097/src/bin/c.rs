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
        k: usize,
    }
    let mut candidate = vec![String::from("~"); k];
    let n = s.len();
    'outer: for i in 0..n {
        'inner: for j in i + 1..=n {
            let t = s[i..j].iter().collect::<String>();
            if t > candidate[k - 1] {
                continue 'outer;
            }
            // eprintln!("{}", t);
            for k in 0..k {
                if candidate[k] == t {
                    continue 'inner;
                }
            }
            for a in 0..k {
                if t < candidate[a] {
                    for b in (a + 1..k).rev() {
                        candidate.swap(b - 1, b);
                    }
                    candidate[a] = t;
                    break;
                }
            }
        }
    }
    // eprintln!("{:?}", candidate);
    println!("{}", candidate[k - 1]);
}

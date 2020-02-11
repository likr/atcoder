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
        n: usize,
        s: Chars,
        t: [Chars; n],
    }
    let mut count = 0;
    'outer: for ti in t {
        for d in 1..(ti.len() + s.len() - 2) / (s.len() - 1) {
            for j in 0..ti.len() - d * (s.len() - 1) {
                let mut ok = true;
                // eprintln!("{} {}", d, j);
                for k in 0..s.len() {
                    if s[k] != ti[j + d * k] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    count += 1;
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", count);
}

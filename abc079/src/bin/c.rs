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
        n: Chars,
    }
    let n = n
        .into_iter()
        .map(|c| (c as usize - '0' as usize) as i64)
        .collect::<Vec<_>>();
    for x in 0..8 {
        let ops = (0..3)
            .map(|i| if x & 1 << i > 0 { '+' } else { '-' })
            .collect::<Vec<_>>();
        let mut s = n[0];
        for j in 0..3 {
            if ops[j] == '+' {
                s += n[j + 1];
            } else {
                s -= n[j + 1];
            }
        }
        if s == 7 {
            println!(
                "{}{}{}{}{}{}{}=7",
                n[0], ops[0], n[1], ops[1], n[2], ops[2], n[3]
            );
            return;
        }
    }
}

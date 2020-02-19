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
        mut n: Chars,
    }
    let m = n.len();
    n.reverse();
    n.push('0');
    let n = n
        .into_iter()
        .map(|d| d as usize - '0' as usize)
        .collect::<Vec<_>>();
    let mut s = 0;
    let mut carry = false;
    for i in 0..m {
        let d = if carry { n[i] + 1 } else { n[i] };
        if d > 5 || (d == 5 && (n[i + 1] >= 5)) {
            s += 10 - d;
            carry = true;
        } else {
            s += d;
            carry = false;
        }
        // eprintln!("{} {}", d, s);
    }
    if carry {
        s += 1;
    }
    println!("{}", s);
}

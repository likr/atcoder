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
        m: usize,
        r: usize,
        mut s: [usize; n - 1],
    }
    s.sort();
    s.reverse();
    let total = (0..k - 1).map(|i| s[i]).sum::<usize>();
    if k < n && total + s[k - 1] >= r * k {
        println!("0");
    } else {
        if total >= r * k {
            println!("0");
        } else if r * k - total <= m {
            println!("{}", r * k - total);
        } else {
            println!("-1");
        }
    }
}

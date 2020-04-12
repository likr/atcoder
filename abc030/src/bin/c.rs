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
        m: usize,
        x: usize,
        y: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut result = 0;
    let mut t = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        while i < n && a[i] < t {
            i += 1;
        }
        if i == n {
            break;
        }
        t = a[i] + x;

        while j < m && b[j] < t {
            j += 1;
        }
        if j == m {
            break;
        }
        t = b[j] + y;

        result += 1;
    }
    println!("{}", result);
}

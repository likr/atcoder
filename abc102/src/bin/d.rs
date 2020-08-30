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
        a: [usize; n],
    }
    let mut acc = vec![0; n];
    acc[0] = a[0] as isize;
    for i in 1..n {
        acc[i] = a[i] as isize + acc[i - 1];
    }
    let mut result = INF as isize;
    let mut i = 0;
    let mut j = 2;
    for k in 1..n - 2 {
        while i + 1 < k && (acc[k] - 2 * acc[i + 1]).abs() < (acc[k] - 2 * acc[i]).abs() {
            i += 1;
        }
        let p = acc[i];
        let q = acc[k] - acc[i];
        while j <= k
            || (j + 1 < n
                && (acc[n - 1] - 2 * acc[j + 1] + acc[k]).abs()
                    < (acc[n - 1] - 2 * acc[j] + acc[k]).abs())
        {
            j += 1;
        }
        let r = acc[j] - acc[k];
        let s = acc[n - 1] - acc[j];
        let mut values = vec![p, q, r, s];
        values.sort();
        result = min(result, values[values.len() - 1] - values[0]);
    }
    println!("{}", result);
}

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
        p: [usize; n],
    }
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i] - 1] = i;
    }

    let mut max_k = 0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && q[j - 1] < q[j] {
            j += 1;
        }
        max_k = max(max_k, j - i);
        i = j;
    }
    println!("{}", n - max_k);
}

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
    for &ai in &a {
        eprintln!("{:24b}", ai);
    }
    let m = 20;
    let mut bits = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            bits[i][j] = a[i] >> j & 1;
        }
    }
    let mut result = 0;
    let mut s = HashSet::new();
    let mut l = 0;
    for r in 0..n {
        if (0..m).any(|j| bits[r][j] == 1 && s.contains(&j)) {
            let k = r - l;
            result += k * (k + 1) / 2;
            s.clear();
            l = r;
        }
        for j in 0..m {
            if bits[r][j] == 1 {
                s.insert(j);
            }
        }
        eprintln!("{}", result);
    }
    let k = n - l;
    result += k * (k + 1) / 2;
    println!("{}", result);
}

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
        b: [usize; n],
    }
    let m = 32;
    let mut a = vec![0; n];
    for i in 0..m {
        for j in 1..n {
            if b[j - 1] & 1 << i != a[j - 1] & 1 << i {
                a[j] |= 1 << i;
            }
        }
        if a[n - 1] & 1 << i == b[n - 1] & 1 << i {
            continue;
        }
        a[0] |= 1 << i;
        for j in 1..n {
            a[j] = a[j] & (2usize.pow(i as u32) - 1);
        }
        for j in 1..n {
            if b[j - 1] & 1 << i != a[j - 1] & 1 << i {
                a[j] |= 1 << i;
            }
        }
        if a[n - 1] & 1 << i == b[n - 1] & 1 << i {
            println!("-1");
            return;
        }
    }
    for &ai in &a {
        println!("{}", ai);
    }
}

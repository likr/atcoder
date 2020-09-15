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
        k: usize,
        a: [usize; k],
    }

    let f = |target| {
        let mut ok = 0;
        let mut ng = INF;
        while ng - ok > 1 {
            let m = (ok + ng) / 2;
            let mut x = m;
            for i in 0..k {
                x = x - x % a[i];
            }
            if x > target {
                ng = m;
            } else {
                ok = m;
            }
        }
        ok
    };

    let l = f(1) + 1;
    let h = f(2);
    if l > h {
        println!("-1");
    } else {
        println!("{} {}", l, h);
    }
}

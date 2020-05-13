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
        d: usize,
        a: [usize; m],
    }
    let mut x = (0..n).collect::<Vec<_>>();
    for &ai in &a {
        x.swap(ai - 1, ai);
    }
    eprintln!("{:?}", x);

    let mut cycle = 1;
    let mut k = 0;
    loop {
        k = x[k];
        cycle += 1;
        if k == 0 {
            break;
        }
    }

    let mut k = 0;
    for _ in 0..d % cycle {
        k = x[k];
    }
    println!("{}", k + 1);
}

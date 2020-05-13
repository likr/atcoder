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
        mut a: [usize; n],
        mut bc: [(usize, usize); m],
    }
    bc.sort_by_key(|&(_, ci)| ci);
    bc.reverse();

    let mut count = 0;
    'outer: for i in 0..m {
        let (bi, ci) = bc[i];
        for _ in 0..bi {
            a.push(ci);
            count += 1;
            if count == n {
                break 'outer;
            }
        }
    }
    a.sort();
    a.reverse();
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    println!("{}", s);
}

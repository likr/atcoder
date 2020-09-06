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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        a: [usize; m],
    }
    let mut x = (0..n).collect::<Vec<usize>>();
    for &ai in &a {
        x.swap(ai - 1, ai);
    }
    debug!(x);

    let mut index = vec![INF; n];
    for i in 0..n {
        index[x[i]] = i
    }
    debug!(index);

    let mut p = vec![index];
    while 2usize.pow(p.len() as u32) <= d {
        let index = &p[p.len() - 1];
        let mut x = vec![0; n];
        for i in 0..n {
            x[index[index[i]]] = i;
        }
        let mut y = vec![0; n];
        for i in 0..n {
            y[x[i]] = i;
        }
        debug!(y);
        p.push(y);
    }

    let mut y = (0..n).collect::<Vec<_>>();
    for k in 0..p.len() {
        if 1 << k & d > 0 {
            y = y.into_iter().map(|i| p[k][i]).collect::<Vec<_>>();
        }
    }
    for i in 0..n {
        println!("{}", y[i] + 1);
    }
}

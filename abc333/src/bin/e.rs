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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        tx: [(usize, Usize1); n],
    }
    let mut x_index = vec![vec![]; n];
    let mut acc = vec![0i64; n];
    let mut ans = HashSet::new();
    for i in 0..n {
        let (ti, xi) = tx[i];
        if ti == 1 {
            x_index[xi].push(i);
        } else {
            if x_index[xi].is_empty() {
                println!("-1");
                return;
            }
            let j = x_index[xi].pop().unwrap();
            ans.insert(j);
            acc[j] = 1;
            acc[i] = -1;
        }
    }
    debug!(acc);
    for i in 1..n {
        acc[i] += acc[i - 1];
    }
    debug!(acc);
    println!("{}", acc.iter().max().unwrap());
    let mut sol = vec![];
    for i in 0..n {
        let (ti, _xi) = tx[i];
        if ti == 1 {
            if ans.contains(&i) {
                sol.push("1");
            } else {
                sol.push("0");
            }
        }
    }
    println!("{}", sol.join(" "));

    let mut count = vec![0; n];
    for i in 0..n {
        let (ti, xi) = tx[i];
        if ti == 1 {
            if ans.contains(&i) {
                count[xi] += 1;
            }
        } else {
            assert!(count[xi] > 0);
            count[xi] -= 1;
        }
    }
}

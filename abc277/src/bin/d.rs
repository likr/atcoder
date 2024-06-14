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
        m: usize,
        mut a0: [usize; n],
    }
    let s = a0.iter().sum::<usize>();
    a0.sort();
    let mut a = a0.clone();
    for &ai in a0.iter() {
        a.push(ai + m);
    }
    a0.dedup();
    if a0.len() == m {
        println!("0");
        return;
    }
    let mut x = 0;
    for i in 0..a.len() {
        if i == 0 || a[i] - a[i - 1] > 1 {
            let mut t = a[i];
            if 2 * i >= a.len() {
                t -= m;
            }
            for j in i + 1.. {
                if j >= a.len() || a[j] - a[j - 1] > 1 {
                    break;
                }
                t += a[j];
                if 2 * j >= a.len() {
                    t -= m;
                }
            }
            x = max(x, t);
        }
    }
    println!("{}", s - x);
}

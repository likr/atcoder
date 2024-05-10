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
        mut a: [[Usize1]; m],
    }
    let mut check = (0..m).collect::<Vec<_>>();
    let mut top = vec![None; n];
    while !check.is_empty() {
        let mut next_check = vec![];
        for &j in check.iter() {
            if let Some(&aj) = a[j].last() {
                if let Some(k) = top[aj] {
                    next_check.push(j);
                    next_check.push(k);
                    a[j].pop();
                    a[k].pop();
                    top[aj] = None;
                } else {
                    top[aj] = Some(j);
                }
            }
        }
        std::mem::swap(&mut check, &mut next_check);
    }
    if (0..m).all(|j| a[j].is_empty()) {
        println!("Yes");
    } else {
        println!("No");
    }
}

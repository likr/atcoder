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
        mut a: [usize; n]
    }
    let mut count2 = vec![0usize; n];
    let mut count3 = vec![0usize; n];
    for i in 0..n {
        while a[i] % 2 == 0 {
            a[i] /= 2;
            count2[i] += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            count3[i] += 1;
        }
    }
    if a.iter().collect::<HashSet<_>>().len() > 1 {
        println!("-1");
        return;
    }
    let min2 = *count2.iter().min().unwrap();
    let min3 = *count3.iter().min().unwrap();
    let mut result = 0;
    for i in 0..n {
        result += count2[i] - min2;
        result += count3[i] - min3;
    }
    println!("{}", result);
}

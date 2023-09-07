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
        q: usize,
        x: [usize; q],
    }
    let mut a = (1..=n).collect::<Vec<_>>();
    let mut indices = a
        .iter()
        .enumerate()
        .map(|(i, &ai)| (ai, i))
        .collect::<HashMap<_, _>>();
    for k in 0..q {
        let i = indices[&x[k]];
        let j = if i == n - 1 { i - 1 } else { i + 1 };
        a.swap(i, j);
        indices.insert(a[i], i);
        indices.insert(a[j], j);
    }
    println!(
        "{}",
        a.iter()
            .map(|ai| format!("{}", ai))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

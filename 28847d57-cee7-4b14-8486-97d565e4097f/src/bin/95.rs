use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        mut s: [String; n],
        t: [String; m],
    }
    s.sort();
    let t = t.into_iter().collect::<HashSet<_>>();
    let m = n - 1;
    let mut k = 16 - m;
    for i in 0..n {
        k -= s[i].len();
    }
    let mut spacer = vec![];
    for i in 0..=k {
        for mut x in 0..m.pow(i as u32) {
            let mut u = vec![1; m];
            for _ in 0..i {
                u[x % m] += 1;
                x /= m;
            }
            spacer.push(u.into_iter().map(|c| "_".repeat(c)).collect::<Vec<_>>());
        }
    }
    spacer.sort();
    spacer.dedup();
    loop {
        for u in spacer.iter() {
            let mut x = vec![];
            for i in 1..n {
                x.push(s[i - 1].clone());
                x.push(u[i - 1].clone());
            }
            x.push(s[n - 1].clone());
            let x = x.join("");
            if x.len() >= 3 && !t.contains(&x) {
                println!("{}", x);
                return;
            }
        }
        if !s.next_permutation() {
            break;
        }
    }
    println!("-1");
}

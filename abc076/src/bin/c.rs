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
        mut s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    if m > n {
        println!("UNRESTORABLE");
        return;
    }
    for i in (0..=n - m).rev() {
        let mut ok = true;
        for j in 0..m {
            if s[i + j] != '?' && s[i + j] != t[j] {
                ok = false;
            }
        }
        if ok {
            for j in 0..m {
                s[i + j] = t[j];
            }
            for j in 0..n {
                if s[j] == '?' {
                    s[j] = 'a';
                }
            }
            println!(
                "{}",
                s.iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<_>>()
                    .join("")
            );
            return;
        }
    }
    println!("UNRESTORABLE");
}

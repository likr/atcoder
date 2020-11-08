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
        k: Usize1,
        s: [String; n],
    }
    let mut count = HashMap::new();
    for word in &s {
        *count.entry(word).or_insert(0) += 1;
    }

    let mut count_count = HashMap::new();
    for &c in count.values() {
        *count_count.entry(c).or_insert(0) += 1;
    }

    let mut words = count.iter().collect::<Vec<_>>();
    words.sort_by_key(|e| e.1);
    words.reverse();
    if count_count[&words[k].1] > 1 {
        println!("AMBIGUOUS");
    } else {
        println!("{}", words[k].0);
    }
}

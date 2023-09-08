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
    }
    let mut boxes = vec![BTreeSet::new(); n];
    let mut cards = HashMap::new();
    for k in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                boxes[j - 1].insert((i, k));
                cards.entry(i).or_insert(BTreeSet::new()).insert(j);
            }
            2 => {
                input! {
                    j: usize,
                }
                println!(
                    "{}",
                    boxes[j - 1]
                        .iter()
                        .map(|&(i, _)| format!("{}", i))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            _ => {
                input! {
                    i: usize,
                }
                println!(
                    "{}",
                    cards[&i]
                        .iter()
                        .map(|&j| format!("{}", j))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
    }
}

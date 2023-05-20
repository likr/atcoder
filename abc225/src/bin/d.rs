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

#[derive(Clone)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut nodes = vec![
        Node {
            prev: None,
            next: None
        };
        n
    ];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            }
            nodes[x].next = Some(y);
            nodes[y].prev = Some(x);
        } else if t == 2 {
            input! {
                x: Usize1,
                y: Usize1,
            }
            nodes[x].next = None;
            nodes[y].prev = None;
        } else {
            input! {
                x: Usize1,
            }
            let mut result = vec![x];
            let mut u = x;
            while let Some(v) = nodes[u].prev {
                result.push(v);
                u = v;
            }
            result.reverse();
            let mut u = x;
            while let Some(v) = nodes[u].next {
                result.push(v);
                u = v;
            }
            println!(
                "{} {}",
                result.len(),
                result
                    .iter()
                    .map(|&u| format!("{}", u + 1))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}

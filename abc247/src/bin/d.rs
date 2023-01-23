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
        q: usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                c: usize,
            }
            queue.push_back((x, c));
        } else {
            input! {
                mut c: usize,
            }
            let mut s = 0;
            while c > 0 {
                let (y, d) = queue.pop_front().unwrap();
                if d >= c {
                    s += y * c;
                    if d > c {
                        queue.push_front((y, d - c));
                    }
                    c = 0;
                } else {
                    s += y * d;
                    c -= d;
                }
            }
            println!("{}", s);
        }
    }
}

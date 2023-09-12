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
        d: usize,
        lr: [(usize, usize); n],
    }
    let mut x = vec![];
    let mut events = vec![];
    for (i, &(li, ri)) in lr.iter().enumerate() {
        x.push(li);
        x.push(ri);
        events.push((li, 1, i));
        events.push((ri, 0, i));
    }
    x.sort();
    x.dedup();
    events.sort();
    let mut j = 0;
    let mut punch = None;
    let mut broken = vec![false; n];
    let mut ans = 0;
    let mut wall = vec![];
    for &xi in x.iter() {
        if let Some(s) = punch {
            if s + d <= xi {
                punch = None;
            }
        }
        while j < events.len() && events[j].0 == xi && events[j].1 == 0 {
            if !broken[events[j].2] {
                if punch.is_none() {
                    for &k in wall.iter() {
                        broken[k] = true;
                    }
                    wall.clear();
                    punch = Some(xi);
                    ans += 1;
                }
                broken[events[j].2] = true;
            }
            j += 1;
        }
        while j < events.len() && events[j].0 == xi && events[j].1 == 1 {
            if punch.is_some() {
                broken[events[j].2] = true;
            } else {
                wall.push(events[j].2);
            }
            j += 1;
        }
    }
    println!("{}", ans);
}

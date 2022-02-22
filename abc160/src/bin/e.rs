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
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        p: [usize; a],
        q: [usize; b],
        r: [usize; c],
    }
    let mut items = vec![];
    for &pi in p.iter() {
        items.push((pi, 0))
    }
    for &qi in q.iter() {
        items.push((qi, 1))
    }
    for &ri in r.iter() {
        items.push((ri, 2))
    }
    items.sort();
    items.reverse();

    let mut result = 0;
    let mut p_count = 0;
    let mut q_count = 0;
    let mut r_count = 0;
    for i in 0..items.len() {
        let (v, t) = items[i];
        match t {
            0 => {
                if p_count < x {
                    result += v;
                    p_count += 1;
                }
            }
            1 => {
                if q_count < y {
                    result += v;
                    q_count += 1;
                }
            }
            _ => {
                result += v;
                r_count += 1;
            }
        }
        if p_count + q_count + r_count == x + y {
            break;
        }
    }
    println!("{}", result);
}

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
        mut h: [i64; n],
    }
    let mut offset = 0;
    let mut odd_count = HashMap::new();
    let mut even_count = HashMap::new();
    for i in 1..n {
        if i % 2 == 1 {
            *odd_count.entry(h[i] - h[i - 1]).or_insert(0) += 1;
        } else {
            *even_count.entry(h[i] - h[i - 1]).or_insert(0) += 1;
        }
    }
    for _ in 0..q {
        debug!(h);
        debug!(odd_count, even_count);
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    vi: i64,
                }
                offset += vi;
            }
            2 => {
                input! {
                    vi: i64,
                }
                offset -= vi;
            }
            3 => {
                input! {
                    ui: Usize1,
                    vi: i64,
                }
                debug!(ui, vi);
                if 0 < ui {
                    let diff = h[ui] - h[ui - 1];
                    debug!(diff);
                    if ui % 2 == 1 {
                        let c = odd_count[&diff];
                        if c == 1 {
                            odd_count.remove(&diff);
                        } else {
                            *odd_count.entry(diff).or_insert(0) -= 1;
                        }
                        *odd_count.entry(diff + vi).or_insert(0) += 1;
                    } else {
                        let c = even_count[&diff];
                        if c == 1 {
                            even_count.remove(&diff);
                        } else {
                            *even_count.entry(diff).or_insert(0) -= 1;
                        }
                        *even_count.entry(diff + vi).or_insert(0) += 1;
                    }
                }
                if ui + 1 < n {
                    let diff = h[ui + 1] - h[ui];
                    debug!(diff);
                    if ui % 2 == 0 {
                        let c = odd_count[&diff];
                        if c == 1 {
                            odd_count.remove(&diff);
                        } else {
                            *odd_count.entry(diff).or_insert(0) -= 1;
                        }
                        *odd_count.entry(diff - vi).or_insert(0) += 1;
                    } else {
                        let c = even_count[&diff];
                        if c == 1 {
                            even_count.remove(&diff);
                        } else {
                            *even_count.entry(diff).or_insert(0) -= 1;
                        }
                        *even_count.entry(diff - vi).or_insert(0) += 1;
                    }
                }
                h[ui] += vi;
            }
            _ => {}
        }
        let mut result = 0;
        if let Some(&c) = odd_count.get(&offset) {
            result += c;
        }
        if let Some(&c) = even_count.get(&-offset) {
            result += c;
        }
        println!("{}", result);
    }
}

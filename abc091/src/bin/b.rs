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

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    }
    let mut s_count = HashMap::new();
    for si in s.iter() {
        *s_count.entry(si).or_insert(0) += 1;
    }
    let mut t_count = HashMap::new();
    for ti in t.iter() {
        *t_count.entry(ti).or_insert(0) += 1;
    }
    let mut result = 0;
    for (si, c) in s_count.iter() {
        if let Some(d) = t_count.get(si) {
            if c > d {
                result = max(result, c - d);
            }
        } else {
            result = max(result, *c);
        }
    }
    println!("{}", result);
}

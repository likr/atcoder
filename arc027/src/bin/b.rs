use petgraph::unionfind::*;
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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn ctoi(c: char) -> usize {
    if 'A' <= c && c <= 'Z' {
        10 + c as usize - 'A' as usize
    } else {
        c as usize - '0' as usize
    }
}

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars,
    }
    let mut unknown = HashSet::new();
    let mut components = UnionFind::new(36);
    for i in 0..n {
        components.union(ctoi(s1[i]), ctoi(s2[i]));
        if ctoi(s1[i]) >= 10 {
            unknown.insert(s1[i]);
        }
        if ctoi(s2[i]) >= 10 {
            unknown.insert(s2[i]);
        }
    }

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digit = "0123456789";

    for d in digit.chars() {
        for a in alphabet.chars() {
            if components.find(ctoi(d)) == components.find(ctoi(a)) {
                unknown.remove(&a);
            }
        }
    }
    debug!(unknown);
    let mut component_count = 0;
    for a in alphabet.chars() {
        if unknown.contains(&a) && components.find(ctoi(a)) == ctoi(a) {
            component_count += 1;
        }
    }
    debug!(component_count);
    let result = if unknown.contains(&s1[0]) {
        9 * 10usize.pow(component_count as u32 - 1)
    } else {
        10usize.pow(component_count as u32)
    };
    println!("{}", result);
}

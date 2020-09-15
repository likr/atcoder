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

fn main() {
    input! {
        k: usize,
        n: usize,
        vw: [(Chars, Chars); n],
    }
    for mut x in 0..3usize.pow(k as u32) {
        let mut l = vec![0; k];
        for i in 0..k {
            l[i] = x % 3 + 1;
            x /= 3;
        }
        debug!(l);
        let mut words = vec![HashSet::new(); k];
        let mut ok = true;
        for i in 0..n {
            let (vi, wi) = &vw[i];
            let mut offset = 0;
            for j in 0..vi.len() {
                let d = vi[j] as usize - '1' as usize;
                if offset + l[d] > wi.len() {
                    ok = false;
                    break;
                }
                words[d].insert(wi[offset..offset + l[d]].iter().collect::<String>());
                offset += l[d];
            }
            if offset != wi.len() {
                ok = false;
            }
        }
        if ok && (0..k).all(|i| words[i].len() == 1) {
            for i in 0..k {
                println!("{}", words[i].iter().nth(0).unwrap());
            }
            return;
        }
    }
}

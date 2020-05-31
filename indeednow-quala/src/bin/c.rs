use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        q: usize,
        k: [usize; q],
    }
    let mut count = HashMap::new();
    count.insert(s.iter().max().unwrap() + 1, 0);
    for i in 0..n {
        if s[i] > 0 {
            *count.entry(s[i]).or_insert(0) += 1;
        }
    }
    let mut t = count.into_iter().collect::<Vec<_>>();
    t.sort_by_key(|&(k, _)| k);
    t.reverse();

    let m = t.len();
    for i in 1..m {
        t[i].1 += t[i - 1].1;
    }

    for i in 0..q {
        let j = t.upper_bound_by_key(&k[i], |&(_, v)| v);
        if j == m {
            println!("0");
        } else {
            println!("{}", t[j].0 + 1);
        }
    }
}

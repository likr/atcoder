use itertools::Itertools;
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

fn combination(n: usize, k: usize) -> usize {
    let k = min(k, n - k);
    let mut f = HashMap::new();
    for i in 0..k {
        let mut v = n - i;
        for i in 2..=v {
            while v % i == 0 {
                *f.entry(i).or_insert(0) += 1;
                v /= i;
            }
        }
    }
    let mut g = HashMap::new();
    for j in 2..=k {
        let mut v = j;
        for i in 2..=v {
            while v % i == 0 {
                *g.entry(i).or_insert(0) += 1;
                v /= i;
            }
        }
    }
    for (v, c) in g.iter() {
        *f.get_mut(v).unwrap() -= c;
    }
    let mut result = 1;
    for (v, c) in f.iter() {
        for _ in 0..*c {
            result *= v;
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut v: [usize; n],
    }
    v.sort();
    v.reverse();
    let vc = v
        .iter()
        .group_by(|&vi| vi)
        .into_iter()
        .map(|(&vi, group)| (vi, group.count()))
        .collect::<Vec<(usize, usize)>>();

    if vc[0].1 >= a {
        let (vi, ci) = vc[0];
        println!("{}", vi);
        let mut c = 0usize;
        for k in a..=min(b, ci) {
            c += combination(ci, k);
        }
        println!("{}", c);
        return;
    }

    let mut sum = 0usize;
    let mut s = 0usize;
    for &(vi, ci) in &vc {
        if s + ci >= a {
            let k = a - s;
            sum += vi * k;
            let avg = sum as f64 / a as f64;
            println!("{}", avg);
            println!("{}", combination(ci, k));
            return;
        } else {
            s += ci;
            sum += vi * ci;
        }
    }
}

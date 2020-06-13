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
        d: usize,
        l: usize,
        n: usize,
        c: [usize; d],
        kft: [(usize, Usize1, usize); n],
    }
    let mut foods = HashMap::new();
    for i in 0..d {
        let ci = c[i];
        foods.entry(ci).or_insert(vec![]).push(i);
    }
    for i in 0..n {
        let (ki, fi, ti) = kft[i];

        if !foods.contains_key(&ki) {
            println!("0");
            continue;
        }

        let list = foods[&ki];
        let k = list.lower_bound(&fi);
        let (start, days) = if k == list.len() {
            (list[0], d - (fi - list[0] + 1))
        } else {
            (list[k], list[k] - fi + 1)
        };
        let count0 = days / l;

        if count0 > ti {
            println!("0");
            continue;
        }

        let mut l = 0;
        let mut r = ti - count0;
        loop {
            let m = (l + r) / 2;
            let x = l * ti / d;
            let y = (l * ti + start) % d;
        }
    }
    println!("{}", n);
}

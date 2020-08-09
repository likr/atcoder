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
        a: [String; n],
    }
    let mut a = a
        .into_iter()
        .map(|ai| {
            let s = ai.split('.').collect::<Vec<_>>();
            if s.len() == 1 {
                (s[0].parse::<usize>().ok().unwrap(), 0)
            } else {
                (
                    format!("{}{}", s[0], s[1]).parse::<usize>().ok().unwrap(),
                    s[1].len(),
                )
            }
        })
        .collect::<Vec<_>>();
    a.sort_by_key(|&(_, k)| k);
    // eprintln!("{:?}", a);

    let mut factors = HashMap::new();
    for i in 0..n {
        let (mut x, k) = a[i];
        let mut two = 0;
        while x % 2 == 0 {
            two += 1;
            x /= 2;
        }
        let mut five = 0;
        while x % 5 == 0 {
            five += 1;
            x /= 5;
        }
        *factors.entry((two, five, k)).or_insert(0) += 1;
    }

    let factors = factors.into_iter().collect::<Vec<_>>();
    eprintln!("{:?}", factors);
    let m = factors.len();
    let mut result = 0usize;
    for i in 0..m {
        let ((fi, gi, ki), ci) = factors[i];
        if fi >= ki && gi >= ki {
            result += ci * (ci - 1) / 2;
        }
    }
    for i in 0..m {
        let ((fi, gi, ki), ci) = factors[i];
        for j in 0..i {
            let ((fj, gj, kj), cj) = factors[j];
            if fi + fj >= ki + kj && gi + gj >= ki + kj {
                result += ci * cj
            }
        }
    }
    println!("{}", result);
}

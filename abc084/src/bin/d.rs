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
        q: usize,
        lr: [(usize, usize); q],
    }
    let r_max = lr.iter().map(|&(_, r)| r).max().unwrap();
    let mut primes = vec![true; r_max + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..primes.len() {
        if !primes[i] {
            continue;
        }
        for j in 2.. {
            if i * j >= primes.len() {
                break;
            }
            primes[i * j] = false;
        }
    }
    let primes = primes
        .into_iter()
        .enumerate()
        .filter(|&(_, f)| f)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let primes_set = primes.iter().collect::<HashSet<_>>();
    let primes2017 = primes
        .iter()
        .filter(|&p| primes_set.contains(&((p + 1) / 2)))
        .map(|&p| p)
        .collect::<BTreeSet<_>>();
    // eprintln!("{:?}", primes2017);
    for &(li, ri) in &lr {
        println!("{}", primes2017.range(li - 1..=ri).count());
    }
}

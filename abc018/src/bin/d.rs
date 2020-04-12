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
        m: usize,
        p: usize,
        q: usize,
        r: usize,
        xyz: [(Usize1, Usize1, usize); r],
    }
    let mut mask = vec![false; n];
    let mut score = vec![0; m];
    let mut result = 0;
    for b in 1..2usize.pow(n as u32) {
        for i in 0..n {
            mask[i] = b >> i & 1 > 0;
        }

        let mut xs = HashSet::new();
        for i in 0..n {
            if mask[i] {
                xs.insert(i);
            }
        }
        if xs.len() > p {
            continue;
        }

        for j in 0..m {
            score[j] = 0;
        }
        for &(xi, yi, zi) in &xyz {
            if xs.contains(&xi) {
                score[yi] += zi;
            }
        }
        score.sort();
        score.reverse();
        let mut total = 0;
        for j in 0..q {
            total += score[j];
        }
        result = max(result, total);
    }
    println!("{}", result);
}

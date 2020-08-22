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
        h: usize,
        w: usize,
        m: usize,
        hw: [(Usize1, Usize1); m],
    }
    let mut h_count = vec![0usize; h];
    for &(hk, _) in &hw {
        h_count[hk] += 1;
    }
    let mut w_count = vec![0usize; w];
    for &(_, wk) in &hw {
        w_count[wk] += 1;
    }
    let max_h = *h_count.iter().max().unwrap();
    let max_w = *w_count.iter().max().unwrap();
    let max_h_count = h_count.iter().filter(|&&x| x == max_h).count();
    let max_w_count = w_count.iter().filter(|&&x| x == max_w).count();
    if hw
        .iter()
        .filter(|&&(hk, wk)| h_count[hk] == max_h && w_count[wk] == max_w)
        .count()
        == max_h_count * max_w_count
    {
        println!("{}", max_h + max_w - 1);
    } else {
        println!("{}", max_h + max_w);
    }
}

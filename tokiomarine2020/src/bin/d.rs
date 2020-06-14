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
        vw: [(usize, usize); n],
        q: usize,
        ul: [(Usize1, usize); q],
    }
    let w_max = 100000;
    let mut states = vec![vec![]; n];
    states[0].push((0, 0));
    states[0].push(vw[0]);
    for i in 1..min(n, 2usize.pow(10)) {
        let (vi, wi) = vw[i];
        let p = (i + 1) / 2 - 1;
        let mut candidates = vec![];
        for &(vp, wp) in &states[p] {
            candidates.push((vp, wp));
            if wi + wp <= w_max {
                candidates.push((vi + vp, wi + wp));
            }
        }
        candidates.sort_by_key(|&(vs, ws)| (ws, Reverse(vs)));
        // eprintln!("{:?}", candidates);
        states[i].push(candidates[0]);
        for j in 1..candidates.len() {
            let (vj, wj) = candidates[j];
            let (vq, _) = states[i][states[i].len() - 1];
            if vj > vq {
                states[i].push((vj, wj));
            }
        }
    }
    for &(u, l) in &ul {
        let result = if states[u].is_empty() {
            let mut vw_local = vec![];
            let mut p = u;
            while states[p].is_empty() {
                vw_local.push(vw[p]);
                p = (p + 1) / 2 - 1;
            }
            // eprintln!("{} {:?}", u, vw_local);
            let m = vw_local.len();
            let mut result = 0;
            for x in 0..2usize.pow(m as u32) {
                let mut s_v = 0;
                let mut s_w = 0;
                for i in 0..m {
                    if x & 1 << i > 0 {
                        s_v += vw_local[i].0;
                        s_w += vw_local[i].1;
                    }
                }
                if s_w <= l {
                    let k = states[p].upper_bound_by_key(&(l - s_w), |&(_, wk)| wk) - 1;
                    result = max(result, states[p][k].0 + s_v);
                }
            }
            result
        } else {
            let k = states[u].upper_bound_by_key(&l, |&(_, wk)| wk) - 1;
            states[u][k].0
        };
        println!("{}", result);
    }
}

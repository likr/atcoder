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
        _n: usize,
        d: usize,
        k: usize,
        lr: [(usize, usize); d],
        st: [(usize, usize); k],
    }
    let mut pos = st.iter().map(|&(si, _)| si).collect::<Vec<_>>();
    let mut finished = vec![INF; k];
    for i in 0..d {
        let (li, ri) = lr[i];
        for j in 0..k {
            let (sj, tj) = st[j];
            if finished[j] != INF {
                continue;
            }
            if li <= pos[j] && pos[j] <= ri {
                if li <= tj && tj <= ri {
                    pos[j] = tj;
                    finished[j] = i + 1;
                } else {
                    if sj < tj {
                        pos[j] = ri;
                    } else {
                        pos[j] = li;
                    }
                }
            }
        }
    }
    for i in 0..k {
        println!("{}", finished[i]);
    }
}

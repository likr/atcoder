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
        w: usize,
        vw: [(usize, usize); n],
    }
    let e = 15;
    if vw.iter().all(|&(_, wi)| wi <= 1000) {
        if vw.iter().map(|&(_, wi)| wi).sum::<usize>() <= w {
            println!("{}", vw.iter().map(|&(vi, _)| vi).sum::<usize>());
        } else {
            let mut dp = vec![vec![0; w + 1]; n + 1];
            for i in 0..n {
                let (vi, wi) = vw[i];
                for j in 0..wi {
                    dp[i + 1][j] = dp[i][j];
                }
                for j in wi..=w {
                    dp[i + 1][j] = max(dp[i][j], dp[i][j - wi] + vi);
                }
            }
            println!("{}", dp[n][w]);
        }
    } else if vw.iter().all(|&(vi, _)| vi <= 1000) {
        let sv = vw.iter().map(|&(vi, _)| vi).sum::<usize>();
        let mut dp = vec![vec![INF; sv + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = 0;
        }
        for i in 0..n {
            let (vi, wi) = vw[i];
            for j in 0..vi {
                dp[i + 1][j] = dp[i][j];
            }
            for j in vi..=sv {
                dp[i + 1][j] = min(dp[i][j], dp[i][j - vi] + wi);
            }
        }
        let mut result = 0;
        for j in 0..=sv {
            if dp[n][j] <= w {
                result = j;
            }
        }
        println!("{}", result);
    } else if n <= e {
        let mut result = 0;
        for x in 0..2usize.pow(n as u32) {
            let mut sv = 0;
            let mut sw = 0;
            for i in 0..n {
                if x & 1 << i > 0 {
                    let (vi, wi) = vw[i];
                    sv += vi;
                    sw += wi;
                }
            }
            if sw <= w {
                result = max(result, sv);
            }
        }
        println!("{}", result);
    } else {
        let mut states1 = vec![];
        for x in 0..2usize.pow(e as u32) {
            let mut sv = 0;
            let mut sw = 0;
            for i in 0..e {
                if x & 1 << i > 0 {
                    let (vi, wi) = vw[i];
                    sv += vi;
                    sw += wi;
                }
            }
            if sw <= w {
                states1.push((sv, sw));
            }
        }
        let mut states2 = vec![];
        for x in 0..2usize.pow((n - e) as u32) {
            let mut sv = 0;
            let mut sw = 0;
            for i in e..n {
                if x & 1 << (i - e) > 0 {
                    let (vi, wi) = vw[i];
                    sv += vi;
                    sw += wi;
                }
            }
            if sw <= w {
                states2.push((sv, sw));
            }
        }

        if states1.is_empty() || states2.is_empty() {
            println!("0");
            return;
        }

        states1.sort_by_key(|&(sv, sw)| (sw, Reverse(sv)));
        let mut new_states1 = vec![];
        new_states1.push(states1[0]);
        for i in 1..states1.len() {
            let (v0, _) = new_states1[new_states1.len() - 1];
            let (v1, w1) = states1[i];
            if v1 > v0 {
                new_states1.push((v1, w1));
            }
        }
        let states1 = new_states1;

        let mut result = 0;
        for &(sv2, sw2) in &states2 {
            let k = states1.upper_bound_by_key(&w, |&(_, sw1)| sw1 + sw2);
            if 0 < k {
                let (sv1, sw1) = states1[k - 1];
                if dbg!(sw1 + sw2) > w {
                    panic!();
                }
                result = max(result, sv1 + sv2);
            }
        }
        println!("{}", result);
    }
}

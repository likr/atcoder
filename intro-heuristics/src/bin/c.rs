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

fn calc_score(t: &Vec<usize>, c: &Vec<isize>, s: &Vec<Vec<isize>>) -> isize {
    let n = c.len();
    let d = t.len();
    let mut last = vec![0; n];
    let mut score = 0;
    for i in 0..d {
        let ti = t[i];
        score += s[i][ti];
        last[ti] = i + 1;
        for j in 0..n {
            score -= c[j] * (i + 1 - last[j]) as isize;
        }
    }
    score
}

fn main() {
    let n = 26;
    input! {
        d: usize,
        c: [isize; n],
        s: [[isize; n]; d],
        mut t: [Usize1; d],
        m: usize,
        dq: [(Usize1, Usize1); m],
    }
    // let mut score = calc_score(&t, &c, &s);
    for &(di, qi) in &dq {
        // let old = t[di];
        t[di] = qi;
        let new_score = calc_score(&t, &c, &s);
        // if new_score <= score {
        //     t[di] = old;
        // } else {
        //     score = new_score;
        // }
        println!("{}", new_score);
    }
}

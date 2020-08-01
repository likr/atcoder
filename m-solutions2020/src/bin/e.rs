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
        xyp: [(isize, isize, usize); n],
    }
    let mut cost = vec![INF; n + 1];
    let m = 2usize.pow(n as u32);
    let mut ch = vec![vec![INF; m]; n];
    let mut cv = vec![vec![INF; m]; n];
    for i in 0..n {
        for x in 0..m {
            ch[i][x] = xyp[i].0.abs() as usize;
            cv[i][x] = xyp[i].1.abs() as usize;
            for j in 0..n {
                if x & 1 << j > 0 {
                    ch[i][x] = min(ch[i][x], (xyp[i].0 - xyp[j].0).abs() as usize);
                    cv[i][x] = min(cv[i][x], (xyp[i].1 - xyp[j].1).abs() as usize);
                }
            }
            ch[i][x] *= xyp[i].2;
            cv[i][x] *= xyp[i].2;
        }
    }
    for x in 0..3usize.pow(n as u32) {
        let mut c = 0;
        let mut y = x;
        let mut zh = 0;
        let mut zv = 0;
        for i in 0..n {
            if y % 3 == 0 {
                c += 1;
                zh |= 1 << i;
            } else if y % 3 == 1 {
                c += 1;
                zv |= 1 << i;
            }
            y /= 3;
        }
        let mut s = 0;
        for i in 0..n {
            s += min(ch[i][zh], cv[i][zv]);
        }
        cost[c] = min(cost[c], s);
    }
    for i in 0..=n {
        println!("{}", cost[i]);
    }
}

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
        a: [usize; n],
        bi: [(usize, [Usize1]); m],
    }
    let mut combo_members = vec![];
    for i in 0..m {
        let mut s = HashSet::new();
        for &i in &bi[i].1 {
            s.insert(i);
        }
        combo_members.push(s);
    }
    let mut result = 0;
    for x in 0..2usize.pow(n as u32) {
        let mut members = vec![];
        for i in 0..n {
            if x & 1 << i > 0 {
                members.push(i);
            }
        }
        if members.len() == 9 {
            let mut score = 0;
            for &i in &members {
                score += a[i];
            }
            for j in 0..m {
                let mut count = 0;
                for &i in &members {
                    if combo_members[j].contains(&i) {
                        count += 1;
                    }
                }
                if count >= 3 {
                    score += bi[j].0;
                }
            }
            result = max(result, score);
        }
    }
    println!("{}", result);
}

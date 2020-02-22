use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let mut chars1 = vec![HashSet::new(); 26];
    let mut chars2 = vec![HashSet::new(); 26];
    for i in 0..n {
        let a = s[i] as usize - 'a' as usize;
        let b = t[i] as usize - 'a' as usize;
        chars1[a].insert(t[i]);
        chars2[b].insert(s[i]);
    }
    for k in 0..26 {
        if chars1[k].len() > 1 || chars2[k].len() > 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

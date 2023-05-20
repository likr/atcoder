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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn rec(s: &Vec<Vec<char>>, indices: &Vec<usize>, h: usize, result: &mut Vec<usize>) {
    let mut groups = vec![vec![]; 26];
    for &i in indices.iter() {
        if h < s[i].len() {
            groups[(s[i][h] as u8 - 'a' as u8) as usize].push(i);
        }
    }
    for j in 0..26 {
        if groups[j].len() >= 2 {
            for &i in groups[j].iter() {
                result[i] = h + 1;
            }
            rec(s, &groups[j], h + 1, result);
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let mut result = vec![0; n];
    rec(&s, &(0..n).collect::<Vec<_>>(), 0, &mut result);
    for i in 0..n {
        println!("{}", result[i]);
    }
}

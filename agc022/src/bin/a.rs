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
        mut s: Chars,
    }
    let n = s.len();
    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();

    if n == 26 {
        let mut t = s.clone();
        if t.next_permutation() {
            let mut k = 0;
            while s[k] == t[k] {
                k += 1;
            }
            println!("{}", (0..=k).map(|i| t[i]).collect::<String>());
        } else {
            println!("-1");
        }
    } else {
        let mut chars = alphabet.iter().collect::<HashSet<_>>();
        for &c in &s {
            chars.remove(&c);
        }
        let mut chars = chars.into_iter().collect::<Vec<_>>();
        chars.sort();
        s.push(*chars[0]);
        println!("{}", s.iter().collect::<String>());
    }
}

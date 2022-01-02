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

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut alphabet_indices = vec![vec![]; 26];
    for i in 0..n {
        let c = s[i] as usize - 'a' as usize;
        alphabet_indices[c].push(i + 1);
    }
    let mut alphabet_position = vec![0; 26];
    let mut result = vec![];
    let mut offset = 0;
    for i in 0..k {
        for j in 0..26 {
            if alphabet_position[j] < alphabet_indices[j].len()
                && n - alphabet_indices[j][alphabet_position[j]] + 1 >= k - i
            {
                result.push((j + 'a' as usize) as u8 as char);
                offset = alphabet_indices[j][alphabet_position[j]];
                break;
            }
        }
        for j in 0..26 {
            while alphabet_position[j] < alphabet_indices[j].len()
                && alphabet_indices[j][alphabet_position[j]] <= offset
            {
                alphabet_position[j] += 1;
            }
        }
    }
    println!("{}", result.iter().collect::<String>());
}

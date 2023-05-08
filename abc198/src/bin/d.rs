use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::Ext;

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
        s: [Chars; 3],
    }
    let mut chars = vec![];
    for i in 0..3 {
        for j in 0..s[i].len() {
            chars.push(s[i][j]);
        }
    }
    chars.sort();
    chars.dedup();
    if chars.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let m = chars.len();
    let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    loop {
        let mut char_num = HashMap::new();
        for i in 0..m {
            char_num.insert(chars[i], nums[i]);
        }
        if s.iter().all(|si| char_num[&si[0]] != 0) {
            let n = s
                .iter()
                .map(|si| {
                    let mut ni = 0usize;
                    for j in 0..si.len() {
                        ni *= 10;
                        ni += char_num[&si[j]];
                    }
                    ni
                })
                .collect::<Vec<_>>();
            if n[0] + n[1] == n[2] {
                println!("{}", n[0]);
                println!("{}", n[1]);
                println!("{}", n[2]);
                return;
            }
        }
        if !nums.next_permutation() {
            break;
        }
    }
    println!("UNSOLVABLE");
}

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

fn f(x: (usize, usize), h: usize, a: &Vec<i64>, dp: &mut HashMap<(usize, usize), bool>) -> bool {
    if let Some(&ans) = dp.get(&x) {
        ans
    } else {
        let masks = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        let ans = if h == 9 {
            let mut s1 = 0;
            let mut s2 = 0;
            for i in 0..9 {
                if x.0 & 1 << i > 0 {
                    s1 += a[i];
                }
                if x.1 & 1 << i > 0 {
                    s2 += a[i];
                }
            }
            s1 > s2
        } else if h % 2 == 0 {
            if masks
                .iter()
                .any(|mask| mask.iter().all(|i| x.1 & 1 << i > 0))
            {
                false
            } else {
                (0..9)
                    .filter(|&i| x.0 & 1 << i == 0 && x.1 & 1 << i == 0)
                    .any(|i| f((x.0 ^ 1 << i, x.1), h + 1, a, dp))
            }
        } else {
            if masks
                .iter()
                .any(|mask| mask.iter().all(|i| x.0 & 1 << i > 0))
            {
                true
            } else {
                (0..9)
                    .filter(|&i| x.0 & 1 << i == 0 && x.1 & 1 << i == 0)
                    .all(|i| f((x.0, x.1 ^ 1 << i), h + 1, a, dp))
            }
        };
        dp.insert(x, ans);
        ans
    }
}

fn main() {
    input! {
        a: [[i64; 3]; 3],
    }
    let a = a.into_iter().flatten().collect::<Vec<_>>();
    let mut dp = HashMap::new();
    if f((0, 0), 0, &a, &mut dp) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

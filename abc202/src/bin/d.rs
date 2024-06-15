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

fn rec(n: usize, r: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&ans) = memo.get(&(n, r)) {
        ans
    } else {
        let ans = if r == 0 {
            1
        } else {
            (n + 1 - r) * rec(n, r - 1, memo) / r
        };
        memo.insert((n, r), ans);
        ans
    }
}

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    }
    let mut memo = HashMap::new();
    let mut ans = vec![];
    for _ in 0..a + b {
        if k > rec(a + b - 1, b, &mut memo) {
            k -= rec(a + b - 1, b, &mut memo);
            ans.push('b');
            b -= 1;
        } else {
            ans.push('a');
            a -= 1;
        }
    }
    println!("{}", ans.iter().collect::<String>());
}

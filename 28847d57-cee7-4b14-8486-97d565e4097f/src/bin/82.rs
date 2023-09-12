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
        mut a: [i64; n],
    }
    a.sort();
    let mut b = VecDeque::new();
    b.push_back(a.pop().unwrap());
    let mut a = a.into_iter().collect::<VecDeque<_>>();
    let mut ans = 0;
    for _ in 1..n {
        let ah = *a.front().unwrap();
        let at = *a.back().unwrap();
        let bh = *b.front().unwrap();
        let bt = *b.back().unwrap();
        let d = [
            (ah - bh).abs(),
            (ah - bt).abs(),
            (at - bh).abs(),
            (at - bt).abs(),
        ];
        let max_d = *d.iter().max().unwrap();
        ans += max_d;
        if d[0] == max_d {
            a.pop_front();
            b.push_front(ah);
        } else if d[1] == max_d {
            a.pop_front();
            b.push_back(ah);
        } else if d[2] == max_d {
            a.pop_back();
            b.push_front(at);
        } else {
            a.pop_back();
            b.push_back(at);
        }
    }
    println!("{}", ans);
}

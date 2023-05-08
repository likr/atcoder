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
        x: Chars,
        m: usize,
    }
    let x = x
        .into_iter()
        .map(|xi| (xi as u8 - '0' as u8) as usize)
        .collect::<Vec<_>>();
    if x.len() == 1 {
        if m >= x[0] {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    let d = x.iter().max().unwrap();
    let mut l = d + 1;
    let mut h = m + 1;
    if h <= l {
        println!("0");
        return;
    }
    {
        let mut v = Some(0usize);
        for i in 0..x.len() {
            v = v
                .and_then(|v| v.checked_mul(l))
                .and_then(|v| v.checked_add(x[i]))
        }
        if v.is_none() || v.unwrap() > m {
            println!("0");
            return;
        }
    }
    while h - l > 1 {
        let mid = (h + l) / 2;
        let mut v = Some(0usize);
        for i in 0..x.len() {
            v = v
                .and_then(|v| v.checked_mul(mid))
                .and_then(|v| v.checked_add(x[i]))
        }
        if v.is_some() && v.unwrap() <= m {
            l = mid;
        } else {
            h = mid;
        }
    }
    println!("{}", h - (d + 1));
}

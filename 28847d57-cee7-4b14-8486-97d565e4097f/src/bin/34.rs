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
        x: i64,
    }
    let m = 1000i64;
    let nums = (0..=m).map(|v| v * v * v * v * v).collect::<Vec<_>>();
    for i in 0..=m {
        for j in 0..=m {
            if nums[i as usize] - nums[j as usize] == x {
                println!("{} {}", i, j);
                return;
            }
            if nums[i as usize] + nums[j as usize] == x {
                println!("{} {}", i, -j);
                return;
            }
            if -nums[i as usize] - nums[j as usize] == x {
                println!("{} {}", -i, j);
                return;
            }
            if -nums[i as usize] + nums[j as usize] == x {
                println!("{} {}", -i, -j);
                return;
            }
        }
    }
}

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
        m: usize,
        a: [usize; n],
    }
    let mut s = 0;
    let mut nums0 = vec![];
    let mut count = HashMap::new();
    for i in 0..n {
        s += a[i];
        nums0.push(a[i]);
        *count.entry(a[i]).or_insert(0) += 1;
    }
    nums0.sort();
    nums0.dedup();
    let k = nums0.len();
    if nums0.len() == m || nums0.len() == 1 {
        println!("0");
        return;
    }

    let mut nums = vec![];
    for &x in nums0.iter() {
        nums.push(x);
    }
    for &x in nums0.iter() {
        nums.push(x);
    }

    let mut acc = vec![0; 2 * k + 1];
    for i in 0..2 * k {
        acc[i + 1] = acc[i] + nums[i] * count[&nums[i]];
    }
    debug!(nums);
    debug!(acc);

    let mut result = INF;
    let mut left = 0;
    while left < 2 * k {
        let mut right = left;
        loop {
            right += 1;
            if right == 2 * k || (nums[right - 1] + 1) % m != nums[right] {
                break;
            }
        }
        debug!(left, right);
        result = min(result, s - (acc[right] - acc[left]));
        left = right;
    }
    println!("{}", result);
}

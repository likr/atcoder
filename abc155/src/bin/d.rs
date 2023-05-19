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
const INF: i64 = std::i64::MAX / 4;
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
        mut k: usize,
        a: [i64; n],
    }
    let mut nums_p = vec![];
    let mut nums_m = vec![];
    for i in 0..n {
        if a[i] > 0 {
            nums_p.push(a[i]);
        }
        if a[i] < 0 {
            nums_m.push(a[i]);
        }
    }
    nums_p.sort();
    nums_m.sort();
    nums_m.reverse();
    let count_p = nums_p.len();
    let count_m = nums_m.len();
    let count_z = n - count_p - count_m;

    let mut count_nums = HashMap::new();
    for i in 0..n {
        *count_nums.entry(a[i]).or_insert(0) += 1;
    }
    let mut index_p = HashMap::new();
    for i in 0..count_p {
        index_p.insert(nums_p[i], i);
    }
    let mut index_m = HashMap::new();
    for i in 0..count_m {
        index_m.insert(nums_m[i], i);
    }

    if k <= count_p * count_m {
        nums_p.reverse();
        let mut l = -INF; // ai * aj < l となる組の数が k 個未満
        let mut h = 0; // ai * aj < h となる組の数が k 個以上
        while h - l > 1 {
            let m = (l + h) / 2;
            let mut count = 0;
            for i in 0..count_m {
                let ai = nums_m[i];
                // ai * aj < m となる組の数
                count += nums_p.upper_bound_by_key(&m, |aj| ai * aj);
            }
            if count < k {
                l = m;
            } else {
                h = m;
            }
        }
        println!("{}", h);
    } else if count_z > 0
        && k - count_p * count_m <= count_z * (n - count_z) + count_z * (count_z - 1) / 2
    {
        println!("0");
    } else {
        k -= count_p * count_m;
        if count_z > 0 {
            k -= count_z * (n - count_z) + count_z * (count_z - 1) / 2;
        }
        let mut l = 0; // ai * aj < l となる組の数が k 個未満
        let mut h = INF; // ai * aj < h となる組の数が k 個以上
        while h - l > 1 {
            let m = (l + h) / 2;
            let mut count = 0;
            for i in 0..count_m {
                let ai = nums_m[i];
                // ai * aj < m となる組の数
                count += nums_m[index_m[&ai] + 1..].upper_bound_by_key(&m, |aj| ai * aj);
                if i == index_m[&ai] && ai * ai < m {
                    count += count_nums[&ai] * (count_nums[&ai] - 1) / 2
                }
            }
            for i in 0..count_p {
                let ai = nums_p[i];
                // ai * aj < m となる組の数
                count += nums_p[index_p[&ai] + 1..].upper_bound_by_key(&m, |aj| ai * aj);
                if i == index_p[&ai] && ai * ai < m {
                    count += count_nums[&ai] * (count_nums[&ai] - 1) / 2
                }
            }
            if count < k {
                l = m;
            } else {
                h = m;
            }
        }
        println!("{}", h);
    }
}

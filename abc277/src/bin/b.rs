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
        s: [Chars; n],
    }
    let chars1 = "HDCS".chars().collect::<HashSet<_>>();
    let chars2 = "A23456789TJQK".chars().collect::<HashSet<_>>();
    let mut set = HashSet::new();
    for i in 0..n {
        if !chars1.contains(&s[i][0])
            || !chars2.contains(&s[i][1])
            || set.contains(&s[i].iter().collect::<String>())
        {
            println!("No");
            return;
        }
        set.insert(s[i].iter().collect::<String>());
    }
    println!("Yes");
}

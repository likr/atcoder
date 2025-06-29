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
        mut x: Chars,
    }
    x.reverse();
    let mut next = vec![0; n + 1];
    for i in 1..=n {
        let k = i.count_ones() as usize;
        next[i] = i % k;
    }
    let mut graph = vec![vec![]; n + 1];
    for i in 1..=n {
        graph[i].push(next[i]);
        graph[next[i]].push(i);
    }
    let mut queue = VecDeque::new();
    let mut distance = vec![INF; n + 1];
    queue.push_back(0);
    distance[0] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    debug!(distance);
    let mut ones = 0;
    for i in 0..n {
        if x[i] == '1' {
            ones += 1;
        }
    }
    let mut ans = vec![];
    if ones == 0 {
        for i in 0..n {
            ans.push(1);
        }
    } else if ones == 1 {
        let ones_p1 = ones + 1;
        let mut base_p1 = 1;
        let mut s_p1 = 0;
        for i in 0..n {
            if x[i] == '1' {
                s_p1 = (s_p1 + base_p1) % ones_p1;
            }
            base_p1 = (base_p1 * 2) % ones_p1;
        }
        let mut base_p1 = 1;
        for i in 0..n {
            if x[i] == '1' {
                ans.push(0);
            } else {
                ans.push(distance[(s_p1 + base_p1) % ones_p1] + 1);
            }
            base_p1 = (base_p1 * 2) % ones_p1;
        }
    } else {
        let ones_p1 = ones + 1;
        let ones_m1 = ones - 1;
        let mut base_p1 = 1;
        let mut base_m1 = 1;
        let mut s_p1 = 0;
        let mut s_m1 = 0;
        for i in 0..n {
            if x[i] == '1' {
                s_p1 = (s_p1 + base_p1) % ones_p1;
                s_m1 = (s_m1 + base_m1) % ones_m1;
            }
            base_p1 = (base_p1 * 2) % ones_p1;
            base_m1 = (base_m1 * 2) % ones_m1;
        }
        let mut base_p1 = 1;
        let mut base_m1 = 1;
        for i in 0..n {
            if x[i] == '1' {
                ans.push(distance[(s_m1 + ones_m1 - base_m1) % ones_m1] + 1);
            } else {
                ans.push(distance[(s_p1 + base_p1) % ones_p1] + 1);
            }
            base_p1 = (base_p1 * 2) % ones_p1;
            base_m1 = (base_m1 * 2) % ones_m1;
        }
    }
    ans.reverse();
    for &x in ans.iter() {
        println!("{}", x);
    }
}

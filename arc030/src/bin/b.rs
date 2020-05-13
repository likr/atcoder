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

fn main() {
    input! {
        n: usize,
        x: Usize1,
        h: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![HashSet::new(); n];
    for &(ai, bi) in &ab {
        graph[ai].insert(bi);
        graph[bi].insert(ai);
    }
    let mut m0 = (0..n).filter(|&i| graph[i].len() > 0).count();
    loop {
        for u in 0..n {
            if u != x && h[u] == 0 && graph[u].len() == 1 {
                let v = *graph[u].iter().last().unwrap();
                graph[v].remove(&u);
                graph[u].clear();
            }
        }
        let m = (0..n).filter(|&i| graph[i].len() > 0).count();
        if m0 == m {
            break;
        }
        m0 = m;
    }
    if m0 == 0 {
        println!("0");
    } else {
        println!("{}", (m0 - 1) * 2);
    }
}

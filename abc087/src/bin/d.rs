use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      m: usize,
      lrd: [(Usize1, Usize1, isize); m],
    }
    let mut in_degree = vec![0; n];
    let mut graph = vec![vec![]; n];
    for &(li, ri, di) in &lrd {
        graph[li].push((ri, di, true));
        graph[ri].push((li, di, false));
        in_degree[ri] += 1;
    }

    let mut position = vec![None; n];
    for u in 0..n {
        if in_degree[u] == 0 && position[u].is_none() {
            // eprintln!("{}", u);
            position[u] = Some(0);
            let mut queue = VecDeque::new();
            queue.push_back(u);
            while let Some(v) = queue.pop_front() {
                let pv = position[v].unwrap();
                for &(w, d, ltor) in &graph[v] {
                    if let Some(pw) = position[w] {
                        if (ltor && pw - pv != d) || (!ltor && pv - pw != d) {
                            println!("No");
                            return;
                        }
                    } else {
                        if ltor {
                            position[w] = Some(pv + d);
                        } else {
                            position[w] = Some(pv - d);
                        }
                        queue.push_back(w);
                    }
                }
            }
        }
    }

    // eprintln!("{:?}", position);
    if position.iter().any(|p| p.is_none()) {
        println!("No");
    } else {
        println!("Yes");
    }
}

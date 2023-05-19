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

#[derive(Clone, PartialEq)]
enum Status {
    Unvisited,
    Temporary,
    Visited,
}

fn visit(
    graph: &Vec<Vec<usize>>,
    u: usize,
    status: &mut Vec<Status>,
    result: &mut Vec<usize>,
) -> Option<()> {
    match status[u] {
        Status::Unvisited => {
            status[u] = Status::Temporary;
            let mut s = Some(());
            for &v in graph[u].iter() {
                if visit(graph, v, status, result).is_none() {
                    s = None;
                    break;
                }
            }
            if s.is_some() {
                status[u] = Status::Visited;
                result.push(u);
                Some(())
            } else {
                None
            }
        }
        Status::Temporary => None,
        Status::Visited => Some(()),
    }
}

fn topological_sort(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut status = vec![Status::Unvisited; n];
    let mut result = vec![];
    for u in 0..n {
        if status[u] == Status::Unvisited {
            if visit(graph, u, &mut status, &mut result).is_none() {
                return None;
            }
        }
    }
    result.reverse();
    Some(result)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (xi, yi) = xy[i];
        graph[xi].push(yi);
    }
    let edges = xy.iter().collect::<HashSet<_>>();
    if let Some(sorted) = topological_sort(&graph) {
        for i in 1..n {
            if !edges.contains(&(sorted[i - 1], sorted[i])) {
                println!("No");
                return;
            }
        }
        println!("Yes");
        let mut nums = vec![0; n];
        for i in 0..n {
            nums[sorted[i]] = i;
        }
        println!(
            "{}",
            nums.iter()
                .map(|u| format!("{}", u + 1))
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("No");
    }
}

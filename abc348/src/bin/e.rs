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

fn dfs(
    u: usize,
    p: Option<usize>,
    graph: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    dp: &mut Vec<(usize, usize)>,
) -> (usize, usize) {
    for &v in graph[u].iter() {
        if Some(v) != p {
            let (x, y) = dfs(v, Some(u), graph, c, dp);
            dp[u].0 += x;
            dp[u].1 += y;
        }
    }
    dp[u].0 += dp[u].1;
    dp[u].1 += c[u];
    dp[u]
}

fn bfs(
    u: usize,
    p: Option<(usize, usize, usize)>,
    graph: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    dp: &Vec<(usize, usize)>,
    ans: &mut Vec<(usize, usize)>,
) {
    let d = graph[u].len();
    let mut dp_u = vec![(0, 0); d];
    for (i, &v) in graph[u].iter().enumerate() {
        if p.is_some() && v == p.unwrap().0 {
            dp_u[i].0 = p.unwrap().1;
            dp_u[i].1 = p.unwrap().2;
        } else {
            dp_u[i].0 = dp[graph[u][i]].0;
            dp_u[i].1 = dp[graph[u][i]].1;
        }
    }
    debug!(u, dp_u);
    let mut dp_l = vec![(0, 0); d + 1];
    for i in 0..d {
        dp_l[i + 1].0 += dp_l[i].0 + dp_u[i].0;
        dp_l[i + 1].1 += dp_l[i].1 + dp_u[i].1;
    }
    let mut dp_r = vec![(0, 0); d + 1];
    for i in (0..d).rev() {
        dp_r[i].0 += dp_r[i + 1].0 + dp_u[i].0;
        dp_r[i].1 += dp_r[i + 1].1 + dp_u[i].1;
    }
    ans[u].0 = dp_l[d].0 + dp_l[d].1;
    ans[u].1 = dp_l[d].1 + c[u];
    for (i, &v) in graph[u].iter().enumerate() {
        if p.is_some() && v == p.unwrap().0 {
            continue;
        }
        bfs(
            v,
            Some((
                u,
                dp_l[i].0 + dp_r[i + 1].0 + dp_l[i].1 + dp_r[i + 1].1,
                dp_l[i].1 + dp_r[i + 1].1 + c[u],
            )),
            graph,
            c,
            dp,
            ans,
        );
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        c: [usize; n],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in ab.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dp = vec![(0, 0); n];
    dfs(0, None, &graph, &c, &mut dp);
    let mut ans = vec![(0, 0); n];
    bfs(0, None, &graph, &c, &dp, &mut ans);
    debug!(ans);
    println!("{}", ans.iter().map(|(x, _)| x).min().unwrap());
}

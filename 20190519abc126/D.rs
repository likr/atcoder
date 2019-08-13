use std::collections::{HashSet, VecDeque};

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(neighbors: &Vec<Vec<(usize, usize)>>, u: usize, d: usize, height: &mut Vec<usize>, visited: &mut HashSet<usize>) {
    if visited.contains(&u) {
        return;
    }
    visited.insert(u);
    height[u] = d;
    for &(v, w) in neighbors[u].iter() {
        dfs(neighbors, v, d + w, height, visited);
    }
}

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, usize); n - 1],
    }

    let mut neighbors = vec![vec![]; n];
    for (u, v, w) in uvw {
        let u = u - 1;
        let v = v - 1;
        neighbors[u].push((v, w));
        neighbors[v].push((u, w));
    }

    let mut visited = HashSet::new();
    let mut height = vec![0; n];
    dfs(&neighbors, 0, 0, &mut height, &mut visited);

    for h in height {
        println!("{}", if h % 2 == 0 { 0 } else { 1 });
    }
}

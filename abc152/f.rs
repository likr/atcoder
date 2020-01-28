use std::cmp::{min, max};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn path(graph: &Vec<Vec<usize>>, edge_id: &HashMap<(usize, usize), u64>, u: usize, v: usize) -> u64 {
    let n = graph.len();
    let mut parent = vec![0; n];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(u);
    while queue.len() > 0 {
        let w = queue.pop_front().unwrap();
        for &x in &graph[w] {
            if !visited.contains(&x) {
                queue.push_back(x);
                visited.insert(x);
                parent[x] = w;
            }
        }
    }
    let mut result = 0u64;
    let mut w = v;
    while w != u {
        result |= edge_id[&(min(w, parent[w]), max(w, parent[w]))];
        w = parent[w];
    }
    result
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n + 1];
    let mut edge_id = HashMap::new();
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        graph[ai].push(bi);
        graph[bi].push(ai);
        edge_id.insert((min(ai, bi), max(ai, bi)), 1u64 << i);
    }
    let paths = uv.iter().map(|&(u, v)| path(&graph, &edge_id, u, v)).collect::<Vec<_>>();
    // for p in &paths {
    //     println!("{:?}", p);
    // }
    let mut result = 0;
    let p = 2usize.pow(m as u32);
    for i in 0..p {
        let mut i = i;
        let mut path_count = 0;
        let mut union = 0u64;
        for j in 0..m {
            if i % 2 == 1 {
                path_count += 1;
                union |= paths[j];
            }
            i /= 2;
        }
        let mut edge_count = 0;
        for i in 0..n - 1 {
            if (union >> i) % 2 == 0 {
                edge_count += 1;
            }
        }
        let c = 2u64.pow(edge_count as u32);
        // println!("{} {:?}", c, union);
        if path_count % 2 == 0 {
            result += c;
        } else {
            result -= c;
        }
    }
    println!("{}", result);
}

use std::collections::{HashSet, VecDeque};

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

fn main() {
    input! {
        n: usize,
        u: usize,
        v: usize,
        ab: [(usize, usize); n - 1]
    }
    let mut adjacency = vec![vec![]; n + 1];
    for (ai, bi) in ab {
        adjacency[ai].push(bi);
        adjacency[bi].push(ai);
    }

    let mut height = vec![0; n + 1];
    let mut parent = vec![0; n + 1];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(v);
    while queue.len() > 0 {
        let w = queue.pop_front().unwrap();
        visited.insert(w);
        for &x in adjacency[w].iter() {
            if !visited.contains(&x) {
                height[x] = height[w] + 1;
                parent[x] = w;
                queue.push_back(x);
            }
        }
    }

    let h = (height[u] - 1) / 2;
    let mut w = u;
    for _ in 0..h {
        w = parent[w];
    }

    let mut max_node = u;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(w);
    while queue.len() > 0 {
        let x = queue.pop_front().unwrap();
        visited.insert(x);
        for &y in adjacency[x].iter() {
            if !visited.contains(&y) && height[y] > height[w] {
                if height[y] > height[max_node] {
                    max_node = y;
                }
                queue.push_back(y);
            }
        }
    }

    println!("{}", height[max_node] - 1);
}

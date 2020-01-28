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

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut adjacency = vec![vec![]; n + 1];
    for &(ai, bi) in &ab {
        adjacency[ai].push(bi);
        adjacency[bi].push(ai);
    }

    let mut degrees = adjacency.iter().enumerate().map(|(i, l)| (i, l.len())).collect::<Vec<_>>();
    degrees.sort_by_key(|&(_, d)| d);
    degrees.reverse();
    let k = degrees[0].1;
    println!("{}", k);

    let mut colors = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(1);

    while queue.len() > 0 {
        let u = queue.pop_front().unwrap();
        visited.insert(u);
        let mut used = 0;
        for &v in &adjacency[u] {
            if colors.contains_key(&(u, v)) {
                used = *colors.get(&(u, v)).unwrap();
                break;
            }
        }
        let mut c = 0;
        for &v in &adjacency[u] {
            if !visited.contains(&v) {
                queue.push_back(v);
                c += 1;
                if c == used {
                    c += 1;
                }
                colors.insert((u, v), c);
                colors.insert((v, u), c);
            }
        }
    }

    for &(ai, bi) in &ab {
        println!("{}", colors.get(&(ai, bi)).unwrap());
    }
}

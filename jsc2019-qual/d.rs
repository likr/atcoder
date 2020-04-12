use std::collections::{HashSet, HashMap};

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
    }
    let mut levels = vec![vec![0; n]; n];
    let mut edges = HashSet::with_capacity(n * (n - 1) / 2);
    for u in 0..n {
        for v in u + 1..n {
            edges.insert((u, v));
        }
    }
    let mut level = 1;
    'outer: while edges.len() > 0 {
        let mut nodes = HashMap::new();
        let &(u, v) = edges.iter().nth(0).unwrap();
        nodes.insert(u, 0);
        nodes.insert(v, 1);
        let mut d = 1;
        let mut e = edges.take(&(u, v)).unwrap();
        'find: loop {
            let (u, v) = e;
            levels[u][v] = level;
            for w in 0..n {
                let e1 = if w < v {
                    (w, v)
                } else {
                    (v, w)
                };
                if !nodes.contains_key(&w) && edges.contains(&e1) {
                    e = edges.take(&e1).unwrap();
                    d += 1;
                    nodes.insert(w, d);
                    continue 'find;
                }
            }
            break;
        }
        for (w, d) in nodes {
            if d % 2 == 1 && edges.contains(&(u, w)) {
                let (u, w) = edges.take(&(u, w)).unwrap();
                levels[u][w] = level;
            }
        }
        level += 1;
    }
    for u in 0..n {
        for v in u + 1..n {
            print!("{} ", levels[u][v]);
        }
        println!("");
    }
}

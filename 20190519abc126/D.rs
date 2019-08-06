use std::collections::HashSet;

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

fn main() {
    input! {
        n: usize,
        uvw: [[usize; 3]; n - 1],
    }

    let mut color = Vec::new();
    color.resize(n, 2);

    let mut neighbors = Vec::new();
    neighbors.resize(n, Vec::new());
    for row in uvw.iter() {
        let u = row[0] - 1;
        let v = row[1] - 1;
        let w = row[2];
        neighbors[u].push((v, w));
        neighbors[v].push((u, w));
    }

    'outer: loop {
        for i in 0..n {
            if color[i] == 2 {
                continue;
            }
            let mut visited = HashSet::new();
            let mut nodes = Vec::new();
            nodes.push(i);
            while nodes.len() > 0 {
                let u = nodes.pop().unwrap();
                if visited.contains(&u) {
                    continue;
                }
                visited.insert(u);
                for vw in neighbors[u].iter() {
                    let v = vw.0;
                    let w = vw.1;
                    if w % 2 == 0 {
                        color[v] = color[i];
                        nodes.push(v);
                    } else {
                        color[v] = if color[i] == 0 { 1 } else { 0 };
                    }
                }
            }
        }
        for i in 0..n {
            if color[i] == 2 {
                color[i] = 0;
                continue 'outer;
            }
        }
        break;
    }

    for c in color.iter() {
        println!("{}", c);
    }
}

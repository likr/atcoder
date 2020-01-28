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
        h: usize,
        w: usize,
        s: [chars; h],
    }

    let inf = 1000000000;
    let n = w * h;
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..h {
        for j in 1..w {
            let u = i * w + j - 1;
            let v = i * w + j;
            if s[i][j - 1] != '#' && s[i][j] != '#' {
                matrix[u][v] = 1;
                matrix[v][u] = 1;
            }
        }
    }
    for j in 0..w {
        for i in 1..h {
            let u = (i - 1) * w + j;
            let v = i * w + j;
            if s[i - 1][j] != '#' && s[i][j] != '#' {
                matrix[u][v] = 1;
                matrix[v][u] = 1;
            }
        }
    }
    // println!("{:?}", matrix);
    let mut distance = vec![vec![inf; n]; n];
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == 1 {
                distance[i][j] = 1;
            }
        }
        distance[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distance[i][j] > distance[i][k] + distance[k][j] {
                    distance[i][j] = distance[i][k] + distance[k][j];
                }
            }
        }
    }
    // println!("{:?}", distance);
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            if i != j && distance[i][j] != inf && distance[i][j] > result {
                result = distance[i][j];
            }
        }
    }
    println!("{}", result);
}

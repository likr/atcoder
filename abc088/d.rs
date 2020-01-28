use std::collections::VecDeque;

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

    let mut b_count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                b_count += 1;
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while queue.len() > 0 {
        let (i, j, d) = queue.pop_front().unwrap();
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if i == h - 1 && j == w - 1 {
            println!("{}", w * h - d - 1 - b_count);
            return;
        }
        if i > 0 && s[i - 1][j] == '.' {
            queue.push_back((i - 1, j, d + 1));
        }
        if i < h - 1 && s[i + 1][j] == '.' {
            queue.push_back((i + 1, j, d + 1));
        }
        if j > 0 && s[i][j - 1] == '.' {
            queue.push_back((i, j - 1, d + 1));
        }
        if j < w - 1 && s[i][j + 1] == '.' {
            queue.push_back((i, j + 1, d + 1));
        }
    }
    println!("-1");
}

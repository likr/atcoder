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
        h: usize,
        w: usize,
        s: [String; h],
    }
    let chars = s.join("").chars().collect::<Vec<_>>();

    let mut vertical = Vec::new();
    vertical.resize(h * w, 0);
    let mut horizontal = Vec::new();
    horizontal.resize(h * w, 0);

    for i in 0..h {
        let mut start = -1i64;
        for j in 0..w {
            if chars[i * w + j] == '#' {
                for k in (start + 1) as usize .. j {
                    horizontal[i * w + k] = (j as i64 - start - 1) as usize;
                }
                start = j as i64;
            }
        }
        for k in (start + 1) as usize .. w {
            horizontal[i * w + k] = (w as i64 - start - 1) as usize;
        }
    }

    for j in 0..w {
        let mut start = -1i64;
        for i in 0..h {
            if chars[i * w + j] == '#' {
                for k in (start + 1) as usize .. i {
                    vertical[k * w + j] = (i as i64 - start - 1) as usize;
                }
                start = i as i64;
            }
        }
        for k in (start + 1) as usize .. h {
            vertical[k * w + j] = (h as i64 - start - 1) as usize;
        }
    }

    let mut max = 0;
    for i in 0..h {
        for j in 0..w {
            if chars[i * w + j] == '.' {
                max = std::cmp::max(max, horizontal[i * w + j] + vertical[i * w + j] - 1);
            }
        }
    }
    println!("{:?}", max);
}

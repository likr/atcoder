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

fn char_index(c: char) -> usize {
    c.to_digit(36).unwrap() as usize - 10
}

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut chars = vec![vec![]; 26];
    for i in 0..s.len() {
        chars[char_index(s[i])].push(i);
    }

    let mut indices = vec![0; t.len()];
    let mut offset = 0;
    for i in 0..t.len() {
        let cs = &chars[char_index(t[i])];
        if cs.len() == 0 {
            println!("-1");
            return;
        }
        let j = match cs.binary_search(&offset) {
            Ok(j) => j,
            Err(j) => j,
        };
        let j = if j >= cs.len() {
            0
        } else {
            j
        };
        indices[i] = cs[j];
        offset = if indices[i] == s.len() - 1 {
            0
        } else {
            indices[i] + 1
        };
    }

    let mut count = 0;
    for i in 1..t.len() {
        if indices[i] <= indices[i - 1] {
            count += 1;
        }
    }
    count *= s.len();
    count += indices[t.len() - 1] + 1;
    println!("{}", count);
}

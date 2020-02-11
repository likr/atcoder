fn main() {
    let stdin = std::io::stdin();
    let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
    let n: String = bytes
        .by_ref()
        .map(|r| r.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    let n: usize = n.parse::<usize>().expect("Parse Error");
    let l: String = bytes
        .by_ref()
        .map(|r| r.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    let l: usize = l.parse::<usize>().expect("Parse Error");

    let mut x: Vec<Vec<char>> = vec![];
    for _ in 0..l {
        x.push(
            bytes
                .by_ref()
                .map(|r| r.unwrap() as char)
                .skip_while(|c| c.is_whitespace())
                .take_while(|&c| c != '\n')
                .collect(),
        );
    }
    let y: Vec<char> = bytes
        .by_ref()
        .map(|r| r.unwrap() as char)
        .take_while(|&c| c != '\n')
        .collect();

    // println!("{} {}", n, l);
    // println!("{:?}", x);
    // println!("{:?}", y);

    let mut k = 0;
    for i in 0..2 * n - 1 {
        if y[i] == 'o' {
            k = i;
            break;
        }
    }
    for i in (0..l).rev() {
        if k > 0 && x[i][k - 1] == '-' {
            k -= 2;
        } else if k + 1 < 2 * n - 1 && x[i][k + 1] == '-' {
            k += 2;
        }
    }
    println!("{}", k / 2 + 1);
}

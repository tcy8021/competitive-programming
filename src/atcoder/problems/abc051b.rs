fn read_input_to_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<T> = input
        .replace("\n", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    v
}

fn main() {
    let iv: Vec<i64> = read_input_to_vec();
    let k = iv[0];
    let s = iv[1];

    let mut ans = 0;
    for x in 0..=k {
        for y in 0..=k {
            if s - x - y >= 0 && s - x - y <= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

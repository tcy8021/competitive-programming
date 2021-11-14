use std::cmp;
use std::io;

fn read_vec() -> Vec<String> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let v: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    v
}

fn main() {
    let _n: i64 = read_vec()[0].parse().unwrap();
    let v: Vec<i64> = read_vec().into_iter().map(|s| s.parse().unwrap()).collect();

    let mut result = 1000000000;
    for mut a in v {
        let mut div_cnt = 0;
        while a % 2 == 0 {
            a /= 2;
            div_cnt += 1;
        }
        result = cmp::min(result, div_cnt);
    }

    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let s = input.replace("\n", "");

    // +が入るパターンの総数
    let n = 2_i128.pow((s.len() - 1) as u32) - 1;

    let mut sum: i128 = 0;

    // i: 「+」を入れるパターン。各ループで i に1が立っている箇所に「+」を入れる。
    for i in 0..=n {
        // tmp: 「+」と「+」の間にある和を保持する変数
        let mut tmp: i128 = 0;
        for (j, c) in s.chars().enumerate() {
            // 最後の数字は必ず足される
            if j == s.len() - 1 {
                sum += 10 * tmp + c.to_digit(10).unwrap() as i128;
                continue;
            }

            tmp *= 10;
            tmp += c.to_digit(10).unwrap() as i128;

            // 「+」を入れる箇所では足す
            if i & (1 << j) > 0 {
                sum += tmp;
                tmp = 0;
            }
        }
    }
    println!("{}", sum);
}

fn main() {
    // 加算クイズ
    println!("1 + 1 = ??");
    println!("?? の値を入力してください：");
    // ユーザからの入力を保持する変数を用意
    let mut ans_input = String::new();
    // 標準入力から1行取得し、 ans_input に代入
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input から trim()で改行を取り除き、parse()で整数(i32)型に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == 1 + 1) {
        println!("正解です！");
    } else {
        println!("不正解です");
    }

    // 減算クイズ
    println!("1 - 4 = ??");
    println!("?? の値を入力してください：");
    // ユーザからの入力を保持する変数を用意
    let mut ans_input = String::new();
    // 標準入力から1行取得し、 ans_input に代入
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input から trim()で改行を取り除き、parse()で整数(i32)型に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == 1 - 4) {
        println!("正解です！");
    } else {
        println!("不正解です");
    }

    println!("i32 が扱えるデータ範囲：{} ~ {}", i32::MIN, i32::MAX);
    println!("u32 が扱えるデータ範囲：{} ~ {}", u32::MIN, u32::MAX);
}

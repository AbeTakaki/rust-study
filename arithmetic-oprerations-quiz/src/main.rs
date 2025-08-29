use rand::Rng;

fn main() {
    // 加算クイズ
    // ランダムな数字を生成
    let op1 = rand::rng().random_range(0..100);
    let op2 = rand::rng().random_range(0..100);

    println!("{} + {} = ??", op1, op2);
    println!("?? の値を入力してください：");
    // ユーザからの入力を保持する変数を用意
    let mut ans_input = String::new();
    // 標準入力から1行取得し、 ans_input に代入
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input から trim()で改行を取り除き、parse()で整数(i32)型に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == op1 + op2) {
        println!("正解です！");
    } else {
        println!("不正解です");
    }

    // 減算クイズ
    // ランダムな数字を生成
    let op1 = rand::rng().random_range(0..100);
    let op2 = rand::rng().random_range(0..100);

    println!("{} - {} = ??", op1, op2);
    println!("?? の値を入力してください：");
    // ユーザからの入力を保持する変数を用意
    let mut ans_input = String::new();
    // 標準入力から1行取得し、 ans_input に代入
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input から trim()で改行を取り除き、parse()で整数(i32)型に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == op1 - op2) {
        println!("正解です！");
    } else {
        println!("不正解です");
    }
}

use rand::Rng;

fn main() {
    // 正解数を数える変数を追加
    let mut num_of_correct = 0;
    // 正解数が3以下の時は繰り返す
    while num_of_correct < 3 {
        // quiz_modeをランダムに決める
        let quiz_mode = rand::rng().random_range(1..=2);
        match quiz_mode {
            1 => loop {
                // 加算モード
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

                if ans_input == op1 + op2 {
                    println!("正解です！");
                    // 正解したら正解数を1増やす
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です");
                }
            },

            2 => loop {
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
                if ans_input == op1 - op2 {
                    println!("正解です！");
                    // 正解したら正解数を1増やす
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です");
                }
            },
            _ => unreachable!(),
        }
    }
    println!("クリア");
}

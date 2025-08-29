#[derive(Debug, Clone, Copy, PartialEq)]
// 列挙型
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
// 構造体
struct Card {
    suit: Suit,
    rank: i32,
}

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
fn main() {
    // Vec の用意
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deck を用意
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }
    // シャッフル
    let mut rng = ThreadRng::default();
    deck.shuffle(&mut rng);
    println!("{:?}", deck);

    // 手札用のVecを用意
    let mut hand: Vec<Card> = Vec::new();
    // 5枚カードを引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?}: {:}", i + 1, card.suit, card.rank);
    }

    // 手札交換
    println!("入れ替えたいカードの番号を入力してください。（例：1 2 3）");
    // ユーザからの入力を入れるための変数
    let mut input = String::new();
    // ユーザからの入力を変数に書き込む
    std::io::stdin().read_line(&mut input).unwrap();
    // Vecに変換
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    // 与えられた数字の箇所をデッキから取り出したカードを交換
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?}: {:}", i + 1, card.suit, card.rank);
    }

    // 役判定
    // フラッシュのチェック
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);
    // ペア数のチェック
    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュです！");
    } else if count == 4 {
        println!("フォーカードです！");
    } else if count == 3 {
        println!("スリーカードです！");
    } else if count == 2 {
        println!("ツーペアです！");
    } else if count == 1 {
        println!("ワンペアです！");
    } else {
        println!("ノーペアです！");
    }
}

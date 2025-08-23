# Rustの学習メモ

## インストールについて
環境：Windows11  
参考：https://qiita.com/y-428/items/4fcad7e73b061d9154f8

## ツールについて
- Cargo
Rust のビルドツール・パッケージマネージャ

- rustc
Rust のコンパイラ

- clippy
静的解析ツール（linter）の一つ。コードを分析してよりよい書き方やよくある誤りを指摘してくれるツール

- rustfmt
Rust のコードで推奨されている形に自動で整形してくれるツール

- rustdoc
Rust のコード内にコメントを書くことで、それをHTMLドキュメントとして整形してくれるツール

## Rustファイルの命名規則
- Rusrファイルは常に .rs という拡張子をつける
- Rustでは2単語以上を使う場合、[ hello_world.rs ] とアンダースコアを使う必要がある

## Hello World!
- コンパイルを行う  
```
$ rustc ./main.rs
```

- 実行
```
$ ./main
```
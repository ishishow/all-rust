# all-rust
## 発表資料
https://docs.google.com/presentation/d/1C_-vkfuOhVFSszZO-8VYC0kB7dsTtbDQHbIF-sMOYGc/edit?usp=sharing

## Overview

クライアントからサーバーサイドまで全て Rust でかく技術の無駄開発
内容はルーティング、ユーザーログイン、DB アクセスまで入れた割とフルスタックな Todo サイト

## Tech

- client-side
  - Yew(rust を wasm で使えるようにしてくれる)
    - yew-router(ルーティング)
- server-side
  - actix-web
    - diesel
  - postgre

## Specification

### Ver1.0

- ユーザーは考慮せず、ルーティングを使用し、api を介した todo 作成が行えるサービス
  - "todo/"などで新規作成時に api リクエスト,レスポンス(create delete だけで良い)
  - rust で簡単なサーバーサイド、クライアントサイドを書く
  - DB アクセスする
- 課題点
  - actix-web のバージョンが古く、バージョンをあげるとソースコードの変更が必要

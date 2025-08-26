# roBa-RMK

[roBa](https://github.com/kumamuk-git/roBa) のRMKファームウェアによる実装です。

> [!WARNING]
> - この実装は非公式です。原作者様への問い合わせはお控えください。
> - 自己責任で使用してください。このファームウェアを使用することによるハードウェアの破損等について、責任を負いません。
> - この実装は不完全です。特に、トラックボールやロータリーエンコーダーは未実装です。

## 実装状況

- [x] 基本的なキー入力
  - [x] 右手側 (Central)
  - [x] 左手側 (Peripheral)
- [x] Vial対応
- [x] BLE対応
  - [ ] バッテリー情報の取得
- [ ] トラボ対応
- [ ] ロータリーエンコーダー対応

## ローカルビルド手順

1. リポジトリをクローンする
   ```shell
   git clone https://github.com/kot149/roBa-rmk.git
   cd roBa-rmk
   ```
2. [Rustup](https://www.rust-lang.org/ja/tools/install) をインストールする
3. nrf52840用のビルドターゲットを追加する
   ```shell
   rustup target add thumbv7em-none-eabihf
   ```
4. rmkit, flip-link, cargo-makeをインストールする
   ```shell
   cargo install rmkit flip-link cargo-make
   ```
5. uf2ファイルをコンパイルする
   ```shell
   cargo make uf2 --release
   ```

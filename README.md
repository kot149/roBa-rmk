# roBa-rmk

[roBa](https://github.com/kumamuk-git/roBa)キーボードの[RMK](https://rmk.rs)ファームウェアです。

> [!WARNING]
> - この実装は非公式です。自己責任で使用してください。このファームウェアを使用することによるハードウェアの損傷等について、作者は責任を負いません。

## 対応状況

- [x] 基本的なキー入力
- [x] BLE対応
- [x] Vial対応
- [x] ロータリーエンコーダー対応
- [x] トラックボール対応
  - [ ] スクロールレイヤー
  - [ ] オートマウスレイヤー
- [x] バッテリー
  - [x] バッテリー稼働
  - [x] バッテリー充電
  - [x] バッテリー残量の取得
    - [x] Central
    - [ ] Peripheral
- [ ] LED
  - [ ] 接続インジケーター
  - [ ] バッテリー残量インジケーター
  - [ ] 充電状態インジケーター

## ビルド済みファイルのダウンロード

[Releases](https://github.com/kot149/roBa-rmk/releases) からダウンロードできます。

## キーマップ変更手順

[VIal](https://get.vial.today) に対応しています。以下のいずれかを使用してキーマップを変更してください。
- [Vial(Web版)](https://vial.rocks)
- [Vial(デスクトップ版)](https://get.vial.today/download/)
- [VIA custom UI for Vial](https://sekigon-gonnoc.github.io/via-custom-ui-for-vial/)

## ビルド手順

### GitHub Actionsによるビルド

GitHub Actionsでビルドできます。ワークフローファイルは[こちら](.github/workflows/build.yml)

### ローカルビルド手順

1. リポジトリをクローンする
   ```shell
   git clone https://github.com/kot149/roBa-rmk.git
   cd roBa-rmk
   ```
2. [Rustup](https://www.rust-lang.org/ja/tools/install)をインストールする
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
   cargo make uf2
   ```
6. uf2ファイルをフラッシュする
   ※Windows・macOSでのみ動作します。

   central(右手側)
   ```shell
   cargo make flash-central
   ```
   peripheral(左手側)
   ```shell
   cargo make flash-peripheral
   ```

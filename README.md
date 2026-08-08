# roBa-rmk

[roBa](https://github.com/kumamuk-git/roBa)キーボードの[RMK](https://rmk.rs)ファームウェアです。

## ビルド済みファイルのダウンロード

[Releases](https://github.com/kot149/roBa-rmk/releases) からダウンロードできます。

## キーマップ変更手順

[VIal](https://get.vial.today) に対応しています。以下のいずれかを使用してキーマップを変更してください。
- [Vial(Web版)](https://vial.rocks)
- [Vial(デスクトップ版)](https://get.vial.today/download/)
- [Pipette](https://github.com/darakuneko/pipette-desktop/)
- [VIA custom UI for Vial](https://sekigon-gonnoc.github.io/via-custom-ui-for-vial/)

## 機能説明

### トラックボールのモード設定

[`src/pointingproccontroller.rs`](src/pointingproccontroller.rs) で設定可能です。

デフォルトでは以下のように設定されています。

- レイヤー0: カーソル移動
- レイヤー1: 上下左右矢印キー
- レイヤー2: 低速モード
- レイヤー6: スクロールモード

### オートマウスレイヤー

[`keyboard.toml`](keyboard.toml) の `auto_mouse_layer` で設定可能です。
デフォルトではレイヤー5、タイムアウト1000msに設定しています。
非マウスキーを押すとオートマウスレイヤーが解除、マウスキーを押すとタイムアウトを延長します。

### 接続インジケーター

接続状態が変化したとき、RGB LEDが約750ms点灯します。起動直後は未接続として赤く点灯します。CentralでBLEプロファイルを切り換えたときは一時的な切断状態を表示せず、切り換え先が未ペアリングなら黄だけを表示し、ペアリング済みなら赤を表示して接続後に青へ切り換えます。

| 色 | Central（右手側） | Peripheral（左手側） |
| --- | --- | --- |
| 青 | ホストに接続済み | Centralに接続済み |
| 黄 | 未ペアリングのホストとのペアリング待ち（Advertising） | 使用しない |
| 赤 | 未接続、またはペアリング済みホストへの再接続待ち（Advertising） | 未接続（Advertisingを含む） |

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
3. Windowsの場合、[LLVMをインストール](https://rust-lang.github.io/rust-bindgen/requirements.html#windows)し、環境変数`LIBCLANG_PATH`を`(LLVMのインストール先)\bin`に設定する
4. nrf52840用のビルドターゲットを追加する
   ```shell
   rustup target add thumbv7em-none-eabihf
   ```
5. rmkit, flip-link, cargo-makeをインストールする
   ```shell
   cargo install rmkit flip-link cargo-make
   ```
6. uf2ファイルをコンパイルする
   ```shell
   cargo make uf2
   ```
7. uf2ファイルをフラッシュする
   ※Windows・macOSでのみ動作します。

   central(右手側)
   ```shell
   cargo make flash-central
   ```
   peripheral(左手側)
   ```shell
   cargo make flash-peripheral
   ```

### テスト

組み込み向け依存関係を無効にし、ホストターゲットでテストします。

```shell
cargo make test
```

#### トラブルシューティング

##### WindowsでClangライブラリが見つからないエラー

エラー内容:
```
Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"
```

解決方法:
[LLVMをインストール](https://rust-lang.github.io/rust-bindgen/requirements.html#windows)し、環境変数`LIBCLANG_PATH`を`(LLVMのインストール先)\bin`に設定してください。

##### Rustcのスタックオーバーフロー

エラー内容:
```
thread 'rustc' (xxxxx) has overflowed its stack
```

解決方法:
環境変数`RUST_MIN_STACK`を18388608に設定してください。

```powershell
$env:RUST_MIN_STACK = "18388608"
```
または
```bash
export RUST_MIN_STACK=18388608
```

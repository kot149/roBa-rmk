# roBa-rmk

[roBa](https://github.com/kumamuk-git/roBa)キーボードの[RMK](https://rmk.rs)ファームウェアです。

## ビルド済みファイルのダウンロード

[Releases](https://github.com/kot149/roBa-rmk/releases) からダウンロードできます。

## ファームウェア書き込み手順

マイコンのリセットボタンを2回押してブートローダーを起動し、キーボードのuf2ファイルをコピーして書き込みます。

Raytac dongleへ書き込む場合は、`INFO_UF2.TXT`に次の2行があることを確認してください。

```text
UF2 Bootloader 0.9.2
SoftDevice: S140 6.1.1
```

`cargo make flash-dongle`を使うと、この情報を検査して対応するUF2ドライブだけへ書き込みます。手動でコピーする場合も同じ確認を行ってください。

Raytac dongleからZMKへ戻す場合は、まず`mdbt50q_rx + settings_reset`のfirmwareを書き込み、起動後に通常のZMK dongle firmwareを書き込みます。`nrf52840-nosd` snippetは使用しないでください。

## キーマップ変更手順

Rynkを使ってキーマップを変更できます。RynkはRMKのネイティブ設定プロトコルで、USB接続とBLE接続に対応しています。

Rynkのホストツールは開発中です。ブラウザで試す場合は[RMKのRynkドキュメント](https://rmk.rs/docs/features/rynk)を参照してください。Chromium系ブラウザでWeb SerialまたはWebHIDを使用します。RynkとVialは同時に有効化できないため、このファームウェアではVialを使用しません。

Rynkのロックされた操作には、`keyboard.toml`の`[host].unlock_keys`で指定したキーを同時に押します。

## Raytac dongle

`dongle/`にはRaytac MDBT50Q-RX用のUSB dongleファームウェアがあります。dongleはキーボードのHIDレポートとRynkフレームをBLE経由で受信し、USBへ中継します。

キーボードのLayer 7にあるdongleキーを短く押すとdongle用の専用bond slotへ切り替わります。5秒長押しするとdongleのbondを消去して、別のdongleを探します。dongleは起動時にpairing windowを開き、pairing後は保存したキーボードへ再接続します。

## 機能説明

### トラックボールのモード設定

[`src/pointingproccontroller.rs`](src/pointingproccontroller.rs) で設定可能です。

デフォルトでは以下のように設定されています。

- レイヤー0: カーソル移動
- レイヤー1: 上下左右矢印キー
- レイヤー2: 低速モード
- レイヤー6: スクロールモード

### オートマウスレイヤー

[`keyboard.toml`](keyboard.toml) の`auto_mouse_layer`で設定可能です。
デフォルトではレイヤー5、タイムアウト10000msに設定しています。
非マウスキーを押すとオートマウスレイヤーが解除、マウスキーを押すとタイムアウトを延長します。

### Split peripheral接続インジケーター

Peripheral側の接続状態が変化したとき、RGB LEDが約750ms点灯します。

| 色 | Peripheral（左手側） |
| --- | --- |
| 青 | Centralに接続済み |
| 赤 | Centralに未接続 |

## ビルド手順

### GitHub Actionsによるビルド

GitHub Actionsでキーボードのcentral、peripheral、Raytac dongleをビルドできます。ワークフローファイルは[こちら](.github/workflows/build.yml)です。

### ローカルビルド手順

1. リポジトリをクローンする
   ```shell
   git clone https://github.com/kot149/roBa-rmk.git
   cd roBa-rmk
   ```
2. [Rustup](https://www.rust-lang.org/ja/tools/install)をインストールする
3. Windowsの場合、[LLVMをインストール](https://rust-lang.github.io/rust-bindgen/requirements.html#windows)し、環境変数`LIBCLANG_PATH`を`(LLVMのインストール先)\\bin`に設定する
4. nrf52840用のビルドターゲットを追加する
   ```shell
   rustup target add thumbv7em-none-eabihf
   ```
5. flip-link、cargo-make、cargo-binutils、cargo-hex-to-uf2をインストールする
   ```shell
   cargo install flip-link cargo-make cargo-binutils cargo-hex-to-uf2
   ```
6. キーボードのuf2ファイルをコンパイルする
   ```shell
   cargo make uf2
   ```
7. Raytac dongleのuf2ファイルをコンパイルする
   ```shell
   cargo make uf2-dongle
   ```
8. uf2ファイルをフラッシュする
   ※キーボードの自動書き込みはWindows・macOSでのみ動作します。Raytac dongleは`cargo make flash-dongle`を使うか、`INFO_UF2.TXT`を確認してからUF2ドライブへコピーしてください。

   central（右手側）
   ```shell
   cargo make flash-central
   ```

   peripheral（左手側）
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

```text
Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"
```

解決方法:
[LLVMをインストール](https://rust-lang.github.io/rust-bindgen/requirements.html#windows)し、環境変数`LIBCLANG_PATH`を`(LLVMのインストール先)\\bin`に設定してください。

##### Rustcのスタックオーバーフロー

エラー内容:

```text
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

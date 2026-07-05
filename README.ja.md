# FileEncryptor

[English](README.md) | [日本語](README.ja.md)

Rust で作られた、安全で使いやすいファイル暗号化・復号ツールです。  
コマンドラインインターフェース（CLI）とグラフィカルユーザーインターフェース（GUI）の両方に対応しています。

## ✨ 特徴

- 🔐 AES-256-GCM による暗号化
- 🛡️ Argon2 を使ったパスワードベースの鍵導出
- 📁 GUI によるファイル選択と保存
- 🧑‍💻 スクリプトや自動化に使える CLI 対応
- 🚀 最小限の依存関係でクロスプラットフォーム対応

## 🛠️ インストール

```bash
git clone https://github.com/nobuo-miura/FileEncryptor.git
cd FileEncryptor
cargo build --release
```

## 🔧 使い方

### CLI

```bash
# ファイルを暗号化する
cargo run -- encrypt <input_file> <output_file> <password>

# ファイルを復号する
cargo run -- decrypt <input_file> <output_file> <password>

# GUI を起動する
cargo run -- gui
```

### GUI

次のコマンドを実行します。

```bash
cargo run -- gui
```

シンプルな画面で、ファイルの選択、パスワードの入力、暗号化・復号を行えます。

## 📂 フォルダ構成

```
.
├── src/
│   ├── main.rs      # CLI のエントリーポイント
│   ├── gui.rs       # GUI の実装
│   └── crypto.rs    # 暗号化・復号ロジック
└── Cargo.toml
```

## 🔒 セキュリティメモ

- 暗号化ごとにランダムな nonce と salt を生成し、AES-256-GCM を使用します
- 鍵は一意の salt を使って Argon2 により導出されます

## 📃 ライセンス

MIT License

# Directory Tree CLI

Rustで作成したディレクトリツリー表示ツールです。指定したディレクトリの構造をツリー形式で表示し、ディレクトリ数の集計も行います。

## 機能

- ディレクトリ構造をツリー形式で視覚的に表示
- 階層ごとのディレクトリ数を集計表示
- 隠しファイル・フォルダの表示/非表示切り替え
- ディレクトリのみ表示モード
- Angularプロジェクト専用モード（開発に不要なフォルダを自動除外）

## インストール

```bash
# リポジトリをクローン
git clone <repository-url>
cd directory-tree-cli

# ビルド
cargo build --release

# インストール（オプション）
cargo install --path .
```

## 基本的な使い方

### カレントディレクトリを表示
```bash
./target/release/directory-tree-cli
```

### 特定のディレクトリを表示
```bash
./target/release/directory-tree-cli /path/to/directory
```

## オプション

| オプション | 短縮形 | 説明 |
|------------|--------|------|
| `--all` | `-a` | 隠しファイル・フォルダも表示する |
| `--dirs-only` | - | ディレクトリだけを表示する（ファイルは除外） |
| `--angular` | - | Angularプロジェクト用モード |
| `--help` | `-h` | ヘルプを表示 |
| `--version` | `-V` | バージョンを表示 |

## 使用例

### 隠しファイルも含めて表示
```bash
./target/release/directory-tree-cli --all
```

### ディレクトリのみ表示
```bash
./target/release/directory-tree-cli --dirs-only
```

### Angularプロジェクト用モード
```bash
./target/release/directory-tree-cli --angular /path/to/angular-project
```

このモードでは以下のディレクトリが自動的に除外されます：
- `.vscode`
- `node_modules`
- `.git`

### 複数オプションの組み合わせ
```bash
./target/release/directory-tree-cli --angular --dirs-only /path/to/project
```

## 出力例

```
├── src
│   ├── app
│   │   ├── components
│   │   └── services
│   └── assets
├── docs
└── tests

――――――――――――――――――――――――――――
【集計結果】
全体ディレクトリ数: 6
階層ごとのディレクトリ数:
  深さ  1: 3 個
  深さ  2: 2 個
  深さ  3: 1 個
```

## 依存関係

- `clap`: コマンドライン引数の解析
- `walkdir`: ディレクトリの再帰的探索
- `anyhow`: エラーハンドリング

## ライセンス

[ライセンス情報を記載]

## 貢献

プルリクエストやイシューの報告を歓迎します。

## 開発者向け

### 開発環境のセットアップ
```bash
# 依存関係のインストール
cargo build

# テスト実行
cargo test

# 開発中の実行
cargo run -- --help
```

### コード構造

- `main()`: エントリーポイント、引数解析と主要ロジック
- `filter_hidden()`: 隠しファイル/フォルダのフィルタリング
- `print_entry()`: ツリー形式での出力
- `is_last_sibling()`: 同階層での最後要素判定

## トラブルシューティング

### 権限エラーが発生する場合
一部のディレクトリにアクセス権限がない場合は、そのディレクトリはスキップされます。

### 大きなディレクトリでの実行が遅い場合
`--dirs-only` オプションを使用してファイルを除外することで、実行速度を向上させることができます。
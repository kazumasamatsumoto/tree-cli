use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// フォルダをツリー形式で表示する CLI
#[derive(Parser)]
#[command(version, about = "Directory tree CLI in Rust")]
struct Args {
    /// 表示したいディレクトリパス (省略時はカレントディレクトリ)
    #[arg(value_name = "PATH", default_value = ".")]
    path: PathBuf,

    /// 隠しファイル・フォルダも表示する
    #[arg(short, long)]
    all: bool,

    /// ディレクトリだけ表示する
    #[arg(long)]
    dirs_only: bool,

    /// Angular プロジェクト用モード。これがついていると
    /// `.vscode`, `node_modules`, `.git` を除外します
    #[arg(long)]
    angular: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Angular モードで除外したいディレクトリ名リスト
    let angular_exclude = vec![".vscode", "node_modules", ".git"];

    // ① ディレクトリ数カウント用のマップ（深さ -> カウント）
    let mut depth_counts: BTreeMap<usize, usize> = BTreeMap::new();
    // ② 全体のディレクトリ数合計カウンタ
    let mut total_dir_count: usize = 0;

    for entry in WalkDir::new(&args.path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            // (A) 隠しファイル/フォルダを除外するか
            if !filter_hidden(e, args.all) {
                return false;
            }
            // (B) Angularモードが true のとき、特定ディレクトリを除外
            if args.angular && e.file_type().is_dir() {
                if let Some(name) = e.file_name().to_str() {
                    if angular_exclude.iter().any(|&ex| ex == name) {
                        return false;
                    }
                }
            }
            true
        })
        .filter_map(Result::ok)
        // (C) --dirs-only が指定されていれば「ディレクトリだけ」を通す
        .filter(|e| !args.dirs_only || e.file_type().is_dir())
    {
        // エントリの深さを計算（ルートからの相対パスのコンポーネント数）
        // 例: ルートが "<base>" だとすると、<base>/src/app なら depth = 2
        let rel_path = entry.path().strip_prefix(&args.path).unwrap();
        let depth = rel_path.components().count();

        // (D) ディレクトリの場合のみ、集計対象にする
        if entry.file_type().is_dir() {
            // (D1) 合計ディレクトリ数をインクリメント
            total_dir_count += 1;
            // (D2) 深さごとカウントを更新
            *depth_counts.entry(depth).or_insert(0) += 1;
        }

        // (E) 既存のツリー形式で出力
        print_entry(&entry, &args.path);
    }

    // ツリー出力後に結果をまとめて表示
    println!();
    println!("――――――――――――――――――――――――――――");
    println!("【集計結果】");
    println!("全体ディレクトリ数: {}", total_dir_count);

    // 深さごと（depth_counts のキーは自動的に昇順ソートされている）
    println!("階層ごとのディレクトリ数:");
    for (depth, count) in &depth_counts {
        println!("  深さ {:>2}: {} 個", depth, count);
    }

    Ok(())
}

/// 隠しファイル/フォルダを除外するかどうか
fn filter_hidden(entry: &DirEntry, show_all: bool) -> bool {
    if show_all {
        true
    } else {
        entry
            .file_name()
            .to_str()
            .map(|s| !s.starts_with('.'))
            .unwrap_or(true)
    }
}

/// エントリをツリー形式で表示
fn print_entry(entry: &DirEntry, base: &PathBuf) {
    // ルートパスとの相対パスを取得
    let rel = entry.path().strip_prefix(base).unwrap();
    let depth = rel.components().count();

    // インデント用プレフィックスを生成
    let prefix = if depth == 1 {
        String::new()
    } else {
        "    ".repeat(depth - 1)
    };

    // 同じ階層内で最後の要素かどうかを判定
    let is_last = is_last_sibling(entry);
    let branch = if is_last { "└── " } else { "├── " };

    println!("{}{}{}", prefix, branch, entry.file_name().to_string_lossy());
}

/// 同じフォルダ内で最後の要素かどうかを判定
fn is_last_sibling(entry: &DirEntry) -> bool {
    if let Some(parent) = entry.path().parent() {
        if let Ok(entries) = std::fs::read_dir(parent) {
            // ※ここでは隠しファイルやAngular除外は考慮せずシンプルに最後かを判定
            let names: Vec<_> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name())
                .collect();
            return names.last().map(|n| n == entry.file_name()).unwrap_or(false);
        }
    }
    false
}

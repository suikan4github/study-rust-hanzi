use clap::Parser;

/// 漢字学習プログラム
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version)]
struct Args {
    // 今後必要に応じてオプションを追加
}

fn main() {
    let _args = Args::parse();
}

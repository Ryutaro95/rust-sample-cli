use thiserror::Error;

// thiserrorで拡張されることを宣言する
#[derive(Error, Debug)]
enum MyError {
    // エラーの出力を定義
    #[error("failed to read string from {0}")]
    ReadError(String),

    // 発生元のエラーをそのまま使用可能
    #[error(transparent)]
    // #[from]アトリビュートをエラー型の前の指定すると、自動的に From<T>トレイトも実装される
    ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        .map_err(|_| MyError::ReadError(path.into()))?;
    
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::from) // fromで受けられる
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
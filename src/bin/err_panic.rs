use std::fmt;

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;

    num_str // &str型
        .trim() // 文字列前後の空白文字を削除する
        .parse::<i32>() // &str を i32 に変換 戻り値は Result<i32, ParseIntError>
        .map(|t| t * 2) // parse()の結果がOk(t)の場合のみ t * 2 して 戻り値は Ok(t * 2)
        .map_err(|e| MyError::Num(e)) // parse()の結果が Err(e) の場合のみ e を文字列ににして返す
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("parse Error: {}", cause),
        },
    }
}

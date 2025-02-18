use std::{self, fs, str};

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("The divided number is zero."));
    }

    return Ok(a / b);
}

pub fn result_handler_print() {
    println!("\n错误处理练习");

    // 第一题 基本的Rust的使用
    println!("================ No.1: =======================");
    let ret = divide(20, 5);
    match ret {
        Ok(y) => {
            println!("The result is {y}");
        },
        Err(n) => {
            println!("The error is ({n})");
        }
    }
    
    // 第二题 读文件的错误处理
    println!("================ No.2: =======================");
    let content = read_config("./main.rs");
    match content {
        Ok(y) =>  {
            println!("The content is {y}");
        },
        Err(n) => {
            println!("The error is {n}");
        }
    }
    
    let content = read_config_selfdefine_error("./main.rs");
    match content {
        Ok(y) =>  {
            println!("The content is {y}");
        },
        Err(n) => {
            match n {
                ReadFileError::EmptyFile(empty_file_str) =>  {
                    println!("The error is {empty_file_str}");
                },
            }
        }
    }
    
    println!("================ No.3: =======================");
    let ret = parse_age("-5");
    match ret {
        Ok(y) => {
            println!("The age is {y}");
        },
        Err(n) => {
            println!("The error is {n}");
        }
    }
    
    let ret = parse_age("110");
    match ret {
        Ok(y) => {
            println!("The age is {y}");
        },
        Err(n) => {
            println!("The error is {n}");
        }
    }
    
    let ret = parse_age("abc");
    match ret {
        Ok(y) => {
            println!("The age is {y}");
        },
        Err(n) => {
            println!("The error is {n}");
        }
    }
    
    let ret = parse_age("10");
    match ret {
        Ok(y) => {
            println!("The age is {y}");
        },
        Err(n) => {
            println!("The error is {n}");
        }
    }

    println!("================ No.4: =======================");
    match read_number_from_file("src/main0.rs") {
        Ok(y) => {
            println!("The number is {y}");
        },
        Err(n) => {
            println!("The error is [{n}]");
        }
    }
    
    match read_number_from_file("src/main.rs") {
        Ok(y) => {
            println!("The number is {y}");
        },
        Err(n) => {
            println!("The error is [{n}]");
        }
    }

    println!("================ No.5: =======================");
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    match find_first_even(&nums) {
        Some(ret) => {
            match ret {
                Ok(y) => {
                    println!("The first even number is {y}.");
                },
                Err(n) => {
                    println!("{n}");
                }
            }
        },
        None => {
            println!("The vector is empty.");
        }
    }

    let nums: Vec<i32> = vec![];
    match find_first_even(&nums) {
        Some(ret) => {
            match ret {
                Ok(y) => {
                    println!("The first even number is {y}.");
                },
                Err(n) => {
                    println!("{n}");
                }
            }
        },
        None => {
            println!("The vector is empty.");
        }
    }
}

/// 读文件使用 ? 传播错误处理
fn read_config(path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path)?;
    return Ok(content);
}

/// 自定义返回错误枚举
enum ReadFileError {
    EmptyFile(String)
}

fn read_config_selfdefine_error(path: &str) -> Result<String, ReadFileError> {
    let content = fs::read_to_string(path);
    match content {
        Ok(y) => {
            return Ok(y);
        }
        Err(_) => {
            return Err(ReadFileError::EmptyFile(String::from("Not find file.")));
        }
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat(String),
    OutOfRange(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(str) => {
                return write!(f, "Invalid format: {str}");
            },
            ParseError::OutOfRange(str) => {
                return write!(f, "The {str} is out of the range(0, 100).");
            },
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn parse_age(s: &str) -> Result<u8, ParseError> {
    let ret = s.parse();
    match ret {
        Ok(num) => {
            if num > 100 {
                return Err(ParseError::OutOfRange(String::from(s)));
            } else {
                return Ok(num);
            }
        },
        Err(_) => {
            Err(ParseError::InvalidFormat(String::from(s)))
        }
    }
}

// 系统错误处理与自定义错误处理的映射
#[derive(Debug)]
enum MyError {
    IoError(String),
    FormatError(String),
    OtherError,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::FormatError(s) => {
                write!(f, "Read {s} file to parse number but the content format is not number.")
            },
            MyError::IoError(s) => {
                write!(f, "Not found file({s})")
            },
            _ => {
                write!(f, "Other error")
            }
        }
    }
}

impl std::error::Error for MyError {}

fn read_number_from_file(path: &str) -> Result<i32, MyError> {
    let num = match fs::read_to_string(path) {
        Ok(y) => y,
        Err(n) => {
            match n.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(MyError::IoError(String::from(path)));
                },
                _ => {
                    return Err(MyError::OtherError);
                }
            }
            
        }
    };

    let num = num.parse::<i32>();
    match num {
        Ok(y) => Ok(y),
        Err(_) => {
            return Err(MyError::FormatError(String::from(path)));
        }
    }
}

// 寻找第一个偶数，组合 Option 和 Resutl 的方式，支持错误返回
fn find_first_even(nums: &[i32]) -> Option<Result<i32, &str>> {
    if nums.is_empty() {
        return None;
    }

    for x in nums {
        if x % 2 == 0 {
            return Some(Ok(*x))
        }
    }

    return Some(Err("Not found even number."));
}

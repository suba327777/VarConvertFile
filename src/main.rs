mod case;
use clap::{Parser, ValueEnum};
use std::fs::{read_to_string, write};

#[derive(Debug, Parser)]
struct ConvertArgs {
    case: Case,
    path: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum Case {
    Camel,
    Snake,
}

fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            result.push(if capitalize_next {
                c.to_uppercase().next().unwrap()
            } else {
                c
            });
            capitalize_next = false;
        }
    }
    result
}

//ファイルの中身を１行ずつ読み込む関数
fn convert_file_content(state: ConvertArgs) {
    match read_to_string(&state.path) {
        Ok(content) => {
            let converted_content = content
                .lines()
                .map(|line| match state.case {
                    Case::Camel => snake_to_camel(line),
                    Case::Snake => snake_to_camel(line),
                })
                .collect::<Vec<String>>()
                .join("\n");

            if let Err(e) = write(&state.path, converted_content) {
                println!("Failed {}", e)
            } else {
                println!("File success fully writtern!!!!!");
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

// fn write_file()

fn main() {
    let args = ConvertArgs::parse();
    convert_file_content(args);
}

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

fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_upper = false;

    for c in s.chars() {
        if c.is_uppercase() {
            if !last_was_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            last_was_upper = true;
        } else {
            result.push(c);
            last_was_upper = false
        }
    }
    result
}

fn convert_file_content(state: ConvertArgs) {
    match read_to_string(&state.path) {
        Ok(content) => {
            let converted_content = content
                .lines()
                .map(|line| match state.case {
                    Case::Camel => snake_to_camel(line),
                    Case::Snake => camel_to_snake(line),
                })
                .collect::<Vec<String>>()
                .join("\n");

            write_file(&state.path, converted_content);
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

fn write_file(path: &str, converted_content: String) {
    if let Err(e) = write(path, converted_content) {
        println!("Failed {}", e)
    } else {
        println!("File success fully writtern!!!!!");
    }
}

fn main() {
    let args = ConvertArgs::parse();
    convert_file_content(args);
}

#[cfg(test)]
mod test {
    use crate::camel_to_snake;
    use crate::snake_to_camel;

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(camel_to_snake("camelCase"), "camel_case");
        assert_eq!(camel_to_snake("snake_case"), "snake_case");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(snake_to_camel("snake_case"), "snakeCase");
        assert_eq!(snake_to_camel("snakeCase"), "snakeCase");
    }
}

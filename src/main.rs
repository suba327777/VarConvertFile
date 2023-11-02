mod case;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct ConvertArgs {
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

fn run(state: ConvertArgs) {
    match state.case {
        Case::Camel => {
            println!("{:?}", snake_to_camel("snake_case"));
        }
        Case::Snake => {
            println!("snake")
        }
    }
}

fn main() {
    let args = ConvertArgs::parse();
    run(args)
}

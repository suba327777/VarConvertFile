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

fn main() {
    let test = "snake_case";
    println!("{:?}", snake_to_camel(test));
}

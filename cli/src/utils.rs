pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);

    // Read input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the newline character from the end of the input
    let input = input.trim();
    input.to_owned()
}

pub fn to_camel_case(input: &str) -> String {
    let normalized_input = input.replace("-", " ").replace("_", " ");
    let mut words = normalized_input.split_whitespace();
    let mut camel_case_string = String::new();

    if let Some(first_word) = words.next() {
        camel_case_string.push_str(&first_word.to_lowercase());
    }

    for word in words {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            camel_case_string.push(first_char.to_ascii_uppercase());
            camel_case_string.push_str(&chars.as_str().to_lowercase());
        }
    }

    camel_case_string
}

pub fn to_type_case(input: &str) -> String {
    capitalize_first(&to_camel_case(input))
}

pub fn to_lib_case(input: &str) -> String {
    input.to_ascii_lowercase().replace("-", "_")
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

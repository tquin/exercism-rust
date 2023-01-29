
pub fn reverse(input: &str) -> String {

    let mut output = String::new();

    for i in input.chars() {
        output.insert_str(0, &i.to_string());
    }

    return output
}

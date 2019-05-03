fn main() {
    use std::io::Read;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let parse = coffeegrains::parse_file(&input).unwrap();
    let formatted_imports = instantcoffee::format_imports(&parse).unwrap();
    let [start, end] = parse.import_span;
    println!("{}{}{}", &input[..start], formatted_imports, &input[end..]);
}

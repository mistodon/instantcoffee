// TODO: All failures should print the original file

fn main() {
    use std::io::Read;

    let project = coffeegrains::read_project(&std::env::current_dir().unwrap()).unwrap();
    let project = coffeegrains::parse_project(&project).unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut parse = coffeegrains::parse_file(&input, None).unwrap().into_cow();
    println!("{}", parse);
    let [start, end] = parse.import_span;

    let new_imports = instantcoffee::fix_imports(&mut parse, &project).unwrap();

    let formatted_imports = instantcoffee::format_imports(&parse, &[]).unwrap();
    println!("{}{}{}", &input[..start], formatted_imports, &input[end..]);
}

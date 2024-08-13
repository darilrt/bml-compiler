use parser::parser::parse_file;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("Usage: {} <source_file>", args[0]);
        return;
    }

    // Rest of your code goes here
    let file_name = &args[1];

    let ast = parse_file(file_name);

    println!("{:?}", ast);
}

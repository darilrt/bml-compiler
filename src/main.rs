use ir::{ir::IR, semantical_analizer::SemanticalAnalizer};
use parser::parser::parse_file;
use translator::ir_to_html;
mod ir;
mod parser;
mod translator;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("Usage: {} <source_file>", args[0]);
        return;
    }

    let mut print_ast = false;
    let mut print_ir = false;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--print-ast" => print_ast = true,
            "--print-ir" => print_ir = true,
            _ => {
                println!("Unknown option: {}", arg);
                return;
            }
        }
    }

    // Rest of your code goes here
    let file_name = &args[1];

    let ast = parse_file(file_name);

    if print_ast {
        println!("{:#?}", ast);
    }

    let semantical_analizer = SemanticalAnalizer::new();
    let ir = semantical_analizer.analize(&ast);

    if print_ir {
        println!("{:#?}", ir.0);
    }

    let mut translator = ir_to_html::TranslatorHTML {
        output: String::new(),
    };
    let html = translator.translate(&ir.0);

    println!("{}", html);
}

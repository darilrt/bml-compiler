use ir::semantical_analizer::SemanticalAnalizer;
use parser::parser::parse_file;
use translator::ir_to_c;

mod ir;
mod parser;
mod translator;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut print_ast = false;
    let mut print_ir = false;

    // Rest of your code goes here
    let mut file_name = "";

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--print-ast" => print_ast = true,
            "--print-ir" => print_ir = true,
            s => {
                file_name = s;
            }
        }
    }

    if file_name == "" {
        println!("No input file");
        return;
    }

    let ast = parse_file(file_name);

    if print_ast {
        println!("{:#?}", ast);
    }

    let semantical_analizer = SemanticalAnalizer::new();
    let ir = semantical_analizer.analize(&ast);

    if print_ir {
        println!("{:#?}", ir.0);
    }

    let mut translator = ir_to_c::TranslatorC {
        output: String::new(),
    };
    let html = translator.translate(&ir.0);

    println!("{}", html);
}

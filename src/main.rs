use std::env;
use std::fs;
use std::process;
use slate::lexer::Lexer;
use slate::parser::Parser;
use slate::compiler::Compiler;
use slate::server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("\x1b[31mError: Missing input file.\x1b[0m");
                eprintln!("Usage: slate compile <file.slt> [-o <output.json>]");
                process::exit(1);
            }

            let input_file = &args[2];
            let mut output_file = String::new();

            let mut idx = 3;
            while idx < args.len() {
                if args[idx] == "-o" && idx + 1 < args.len() {
                    output_file = args[idx + 1].clone();
                    idx += 2;
                } else {
                    idx += 1;
                }
            }

            if output_file.is_empty() {
                if let Some(pos) = input_file.rfind('.') {
                    output_file = format!("{}.json", &input_file[..pos]);
                } else {
                    output_file = format!("{}.json", input_file);
                }
            }

            println!("\x1b[34m[Slate] Compiling '{}' to '{}'...\x1b[0m", input_file, output_file);
            match compile_to_json(input_file, &output_file) {
                Ok(_) => {
                    println!("\x1b[32m[Slate] Compilation successful! Saved to {}\x1b[0m", output_file);
                }
                Err(e) => {
                    eprintln!("\x1b[31m[Slate] Compilation failed:\x1b[0m");
                    eprintln!("\x1b[31m{}\x1b[0m", e);
                    process::exit(1);
                }
            }
        }
        "watch" => {
            if args.len() < 3 {
                eprintln!("\x1b[31mError: Missing input file to watch.\x1b[0m");
                eprintln!("Usage: slate watch <file.slt> [--port <8080>]");
                process::exit(1);
            }

            let input_file = &args[2];
            let mut port = 8080;

            let mut idx = 3;
            while idx < args.len() {
                if (args[idx] == "--port" || args[idx] == "-p") && idx + 1 < args.len() {
                    if let Ok(p) = args[idx + 1].parse::<u16>() {
                        port = p;
                    }
                    idx += 2;
                } else {
                    idx += 1;
                }
            }

            server::start_server(input_file, port);
        }
        "help" | "-h" | "--help" => {
            print_usage();
        }
        _ => {
            eprintln!("\x1b[31mError: Unknown command '{}'\x1b[0m", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("\x1b[35m=== Slate Universal Visual Language Compiler ===\x1b[0m");
    println!("Usage:");
    println!("  slate compile <file.slt> [-o <output.json>]   Compile Slate code to JSON AST format");
    println!("  slate watch <file.slt> [--port <port>]        Start live reload server and recompile on save");
    println!("  slate help                                    Show this help message");
}

fn compile_to_json(input_path: &str, output_path: &str) -> Result<(), String> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);
    let ast_nodes = parser.parse();

    if !parser.errors.is_empty() {
        let mut err_msg = String::new();
        for err in &parser.errors {
            err_msg.push_str(&format!("{}\n", err));
        }
        return Err(err_msg);
    }

    let compiler = Compiler::new(false);
    let json = compiler.compile(&ast_nodes);

    fs::write(output_path, json)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    Ok(())
}

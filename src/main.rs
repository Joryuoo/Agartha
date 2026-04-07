mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter; 
use std::io::{self, Write};

//hybrid implementation
// main (source, temporary pa) 
//            |
// Lexical Analyzer (lexer.rs)
//           | lexical units(token.rs / Token)
// Syntax Analyzer (parser.rs)
//           | parse tree (ast.rs)
// Interpreter (interpreter.rs) 

fn main() {
    println!("Agartha Test.");
    println!("Type 'exit' on a new line to RUN the program\n");

    //  collect all lines of code
    let mut program_code = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let trimmed = input.trim();

        // stop muna (temporary)
        if trimmed.to_lowercase() == "exit" {
            break;
        }

        // new line sa code
        program_code.push_str(&input);
    }

    println!("\n--- Output ---");

    if program_code.trim().is_empty() {
        println!("Ayo ayo, Migo!");
        return;
    }

    // Now run the compiler pipeline ONCE on the entire block of code!
    match lexer::tokenize(&program_code) {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(program) => {
                    // SUCCESS! Notice there is NO println!("{:#?}", program) here!
                    // It will stay completely silent and just run the code.
                    let mut my_interpreter = interpreter::Interpreter::new();
                    my_interpreter.interpret(program);
                },
                Err(e) => println!("Parser Error: {}", e), 
            }
        }
        Err(e) => println!("Lexer Error: {}", e), 
    }
    
    println!("--- Program Finished ---\n");
}
mod ast;
mod interpreter;
mod parser;
mod scanner;
mod test;
mod token;
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::ast::Expr;
use anyhow::Result;
use clap::Parser as ClapParser;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, Write};
use anyhow::Error;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file_name: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

struct Main {
    scanner: Scanner,
    parser: Parser,
}
impl Main {
    fn run(&mut self, str: &str) {
        self.scanner = Scanner::new(str.to_string());

        self.scanner.scan_tokens();
        // TODO: remove this clone call
        self.parser = Parser::new(self.scanner.tokens.clone());

        let scanner_errors = self.scanner.get_errors(); // -> &Vec<Error>
        let parser_errors = self.parser.get_errors(); // -> &Vec<Error>

        let all_errors: Vec<&Error> = scanner_errors
                                        .iter()
                                        .chain(parser_errors.iter())
                                        .collect();

        if !all_errors.is_empty() {
            println!("{:?}", all_errors);
            return
        }

        let tree = self.parser.parse();

        let result = Expr::interpret(tree);
        println!("{:?}", result);
    }

    fn run_file(&mut self, path: &String) -> Result<()> {
        let mut file: File = File::open(path)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;

        self.run(&contents);
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        let input = std::io::stdin();
        let mut content = String::new();
        loop {
            print!("> ");
            stdout().flush()?;
            content.clear();
            let bytes = input.read_line(&mut content)?;
            if bytes == 0 {
                println!();
                break;
            }
            self.run(&content.trim());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut main = Main {
        scanner: Scanner::default(),
        parser: Parser::default(),
    };

    if let Some(file_name) = &args.file_name {
        main.run_file(file_name)?;
    } else {
        main.run_prompt()?;
    }
    Ok(())
}

mod ast;
mod test;
mod scanner;
mod token;
use crate::scanner::Scanner;
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file_name: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

struct Main {
    scanner: Scanner,
}
impl Main {
    fn run(&mut self, str: &str) {
        self.scanner = Scanner::new(str.to_string());

        self.scanner.scan_tokens();

        if self.scanner.get_errors().is_empty() {
            for token in &self.scanner.tokens {
                println!("{token:?}");
            }
        }
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
    };

    if let Some(file_name) = &args.file_name {
        main.run_file(file_name)?;
    } else {
        main.run_prompt()?;
    }

    if !main.scanner.get_errors().is_empty() {
        for error in main.scanner.get_errors() {
            eprintln!("{error}");
        }
        std::process::exit(65);
    }

    Ok(())
}

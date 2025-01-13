use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file_name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8
}
fn run_file(_path: &String) -> Result<()> {
    Ok(())
}
fn _run_prompt(_path: &String) -> Result<()> {
    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();

    if args.count > 1 {
        println!("too many args");
        return Ok(())
    }
    else if args.count == 1 {
        run_file(&args.file_name)?;
    }
    else {
        todo!();
    }
    println!("{:?}", args);

    Ok(())
}

use clap::Parser;
use shapdf::{execute_instructions, parse_script, Generator};
use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(
    about = "Generate vector shapes into PDF via declarative scripts",
    author,
    version
)]
struct Cli {
    /// Input script file. Use '-' to read from standard input.
    #[arg(value_name = "INPUT")]
    input: Option<String>,

    /// Output PDF file path.
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let (script, inferred_output) = read_script(cli.input.as_deref())?;

    let output_path = cli
        .output
        .or(inferred_output)
        .unwrap_or_else(|| PathBuf::from("output/shapes.pdf"));

    let instructions = parse_script(&script)?;
    let mut generator = Generator::new(output_path.clone());
    execute_instructions(&mut generator, &instructions)?;
    generator.write_pdf()?;
    Ok(())
}

fn read_script(input: Option<&str>) -> Result<(String, Option<PathBuf>), Box<dyn Error>> {
    match input {
        Some("-") => Ok((read_from_stdin()?, None)),
        Some(path) => {
            let path = PathBuf::from(path);
            let script = fs::read_to_string(&path)?;
            let output = path.with_extension("pdf");
            Ok((script, Some(output)))
        }
        None => Ok((read_from_stdin()?, None)),
    }
}

fn read_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

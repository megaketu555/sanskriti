use clap::{Parser, Subcommand};
use sanskriti as imp;
use miette::{IntoDiagnostic, WrapErr};
use std::fs;
use std::path::PathBuf;
use sanskriti::display_banner;
use sanskriti::translate_file_contents;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
    Run { filename: PathBuf },
}

fn main() -> miette::Result<()> {
    display_banner();
    let args = Args::parse();
    match args.command {
        Commands::Tokenize { filename } => {
            let mut any_cc_err = false;

            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            for token in imp::Lexer::new(&file_contents) {
                let token = match token {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{e:?}");
                        if let Some(unrecognized) = e.downcast_ref::<imp::lex::SingleTokenError>() {
                            any_cc_err = true;
                            eprintln!(
                                "[line {}] Error: Unexpected character: {}",
                                unrecognized.line(),
                                unrecognized.token
                            );
                        } else if let Some(unterminated) =
                            e.downcast_ref::<imp::lex::StringTerminationError>()
                        {
                            any_cc_err = true;
                            eprintln!("[line {}] Error: Unterminated string.", unterminated.line(),);
                        }
                        continue;
                    }
                };
                println!("{token}");
            }
            println!("EOF  null");

            if any_cc_err {
                std::process::exit(65);
            }
        }
        Commands::Parse { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            let parser = imp::Parser::new(&file_contents);
            match parser.parse_expression() {
                Ok(tt) => println!("{tt}"),
                Err(e) => {
                    // TODO: match error line format
                    eprintln!("{e:?}");
                    std::process::exit(65);
                }
            }
        }
        Commands::Run { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;
            let translated_contents = imp::translate_file_contents(&file_contents)?;
            let parser = imp::Parser::new(&translated_contents);
            let program = parser.parse_program().unwrap();
            let mut interpreter = imp::Interpreter::new();
            interpreter.eval_program(&program);
        }
    }

    Ok(())
}
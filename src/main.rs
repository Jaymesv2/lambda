use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;

/// A simple lambda calculus evaluator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    // / Name of the person to greet
    //#[arg(short, long)]
    //name: String,

    // / Number of times to greet
    //#[arg(short, long, default_value_t = 1)]
    //count: u8,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Run { files: Vec<String> },
    Build { files: Vec<String> },
}

use lambda::{
    parser::{grammar, tokenizer, Layout, Tokenizer},
    rename::*,
};

fn run_file(prog: &str) {
    let tok = Layout::new(prog, Tokenizer::new(prog));

    let x = tok.clone().filter_map(|x| x.ok()).collect::<Vec<_>>();
    println!("{x:?}");
    //let mut idents = IdentEnv::new();
    let x = grammar::ProgramParser::new().parse(prog, tok.map(tokenizer::to_triple));

    let ast = match x {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to parse: {e:?}");
            return;
        }
    };
    let mut env = Env::new();

    println!("{ast:?}");

    let renamed = rename(ast, &mut env);

    println!("renamed_ast: {renamed:?}");
    //println!("{idents:?}");
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run { files } => {
            for fname in files {
                let mut f = File::open(fname).expect("failed to open file");
                let mut s = String::new();
                f.read_to_string(&mut s).expect("failed to read file");
                run_file(&s);
            }
        }

        /*Commands::Build {
            files
        } => {},*/
        _ => unimplemented!(""),
    }
}

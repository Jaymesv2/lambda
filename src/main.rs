use clap::{Parser, Subcommand};

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
    command: Commands

}
#[derive(Subcommand, Debug)]
enum Commands {
    Run {

    },
    Build {
        files: Vec<String>
    }
}


/*
fn main() {
    let args = Args::parse();

    println!("{args:?}");

    //for _ in 0..args.count {
        //println!("Hello {}!", args.name)
    //}
} 
*/

fn main() {
    use lambda::parser::tokenizer::Tokenizer;
    let program = r#"
        add1 = \x -> x + "hi\n\u{0041}\u{ffff}"
        main = \x -> x
        do_stuff32X = \x -> let a = -532 in x+a
        argfunc x y z = z + y + z
    "#;
    let mut lx = Tokenizer::new(program);
    let mut tokens = Vec::new();

    while let Some(s) = lx.next() {
        if let Ok(s) = s {
            tokens.push(s.1)
        }
    }

    println!("tokens: {tokens:?}");

    let prog = r#" \x -> 1"#;
    let tok = Tokenizer::new(prog);
    let x = lambda::parser::grammar::ExprParser::new().parse(prog, tok);
    println!("{x:?}");
}
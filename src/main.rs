use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    bytecode: String,
    
    #[clap(long, short = 'f')]
    function_call: bool,
}

fn main() {
    let args = Args::parse();

    let mut idx = 2;
    let mut line_count = 0;
    
    if args.function_call {
        /* Print Fxn Selector */
        println!("{}: {}", line_count, &args.bytecode[0..9]);
        idx = 10;
        line_count += 1;
    }

    /* Print Call Data */
    while idx + 64 <= args.bytecode.len() {
        println!("{}: {}", line_count, &args.bytecode[idx..idx+64]);
        idx += 64;
        line_count += 1;
    }
}

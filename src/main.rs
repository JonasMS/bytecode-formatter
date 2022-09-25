use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    bytecode: String,
}

fn main() {
    let args = Args::parse();
    
    /* Print Fxn Selector */
    println!("0: {}", &args.bytecode[0..9]);

    /* Print Call Data */
    let mut idx = 10;
    let mut line_count = 1;
    while idx + 64 <= args.bytecode.len() {
        println!("{}: {}", line_count, &args.bytecode[idx..idx+64]);
        idx += 64;
        line_count += 1;
    }
}

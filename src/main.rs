use std::{env, process};

fn main()
{
    let args: Vec<String> = env::args().collect();
    if let Err(e) = tinygrep::run(&args)
    {
        eprintln!("{}", e);
        process::exit(1);
    };
}

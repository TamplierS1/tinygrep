use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    tinygrep::run(&args);
}

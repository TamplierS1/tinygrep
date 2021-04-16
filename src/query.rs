use std::fs;
use std::io::ErrorKind;

pub struct Query<'a>
{
    pub search_str: &'a str,
    pub filename: &'a str,
    pub contents: String,
}

impl Query<'_>
{
    pub fn new(args: &Vec<String>) -> Query
    {
        Query::check_args(args);

        Query {
            search_str: &args[1].trim(),
            filename: &args[2],
            contents: Query::read_contents(&args[2]),
        }
    }

    fn read_contents(filename: &str) -> String
    {
        match fs::read_to_string(filename)
        {
            Ok(string) => string,
            Err(e) =>
            {
                if e.kind() == ErrorKind::NotFound
                {
                    panic!("Error: file {} does not exist.", filename);
                }
                else
                {
                    panic!("Error: something went wrong.")
                }
            }
        }
    }

    fn check_args(args: &Vec<String>)
    {
        if args.len() != 3
        {
            panic!(
                "Error: {} arguments were provided when 3 are required.",
                args.len()
            )
        }
    }
}

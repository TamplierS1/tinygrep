use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct FileQuery<'a>
{
    pub search_str: &'a str,
    pub path: &'a Path,
    pub name: &'a str,
    pub contents: String,
}

#[derive(Clone)]
pub struct DirQuery<'a>
{
    pub search_str: &'a str,
    pub path: &'a Path,
    pub name: &'a str,
}

impl FileQuery<'_>
{
    pub fn new(args: &Vec<String>) -> FileQuery
    {
        FileQuery::check_args(args);
        let path = Path::new(&args[2]);

        FileQuery {
            search_str: &args[1].trim(),
            path,
            name: path.file_name().unwrap().to_str().unwrap(),
            contents: FileQuery::read_contents(path),
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

        let path = Path::new(&args[2]);

        if !path.exists()
        {
            panic!(
                "Error: {} does not exist.",
                String::from(path.to_str().unwrap())
            );
        }

        if !path.is_file()
        {
            panic!(
                "Error: {} is not a file.",
                String::from(path.to_str().unwrap())
            );
        }
    }
}

impl FileQuery<'_>
{
    fn read_contents(filename: &Path) -> String
    {
        fs::read_to_string(filename).expect("Error: something went wrong.")
    }
}

impl DirQuery<'_>
{
    pub fn new(args: &Vec<String>) -> DirQuery
    {
        DirQuery::check_args(args);
        let path = Path::new(&args[2]);

        DirQuery {
            search_str: &args[1].trim(),
            path,
            name: path.file_name().unwrap().to_str().unwrap(),
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

        let path = Path::new(&args[2]);

        if !path.exists()
        {
            panic!(
                "Error: {} does not exist.",
                String::from(path.to_str().unwrap())
            );
        }

        if !path.is_dir()
        {
            panic!(
                "Error: {} is not a directory.",
                String::from(path.to_str().unwrap())
            );
        }
    }
}

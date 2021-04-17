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
    pub fn new(args: &Vec<String>) -> Result<FileQuery, String>
    {
        FileQuery::check_args(args)?;
        let path = Path::new(&args[2]);

        Ok(FileQuery {
            search_str: &args[1].trim(),
            path,
            name: path.file_name().unwrap().to_str().unwrap(),
            contents: FileQuery::read_contents(path)?,
        })
    }

    fn check_args(args: &Vec<String>) -> Result<(), String>
    {
        let path = Path::new(&args[2]);

        if !path.is_file()
        {
            return Err(format!(
                "Error: {} is not a file.",
                String::from(path.to_str().unwrap())
            ));
        }

        Ok(())
    }

    fn read_contents(filename: &Path) -> Result<String, String>
    {
        match fs::read_to_string(filename)
        {
            Ok(contents) => Ok(contents),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl DirQuery<'_>
{
    pub fn new(args: &Vec<String>) -> Result<DirQuery, String>
    {
        DirQuery::check_args(args)?;
        let path = Path::new(&args[2]);

        Ok(DirQuery {
            search_str: &args[1].trim(),
            path,
            name: path.file_name().unwrap().to_str().unwrap(),
        })
    }

    fn check_args(args: &Vec<String>) -> Result<(), String>
    {
        let path = Path::new(&args[2]);

        if !path.is_dir()
        {
            return Err(format!(
                "Error: {} is not a directory.",
                String::from(path.to_str().unwrap())
            ));
        }

        Ok(())
    }
}

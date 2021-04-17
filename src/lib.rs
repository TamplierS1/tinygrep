#![feature(in_band_lifetimes)]

mod query;
mod result;

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};

use colored::Colorize;

use query::{DirQuery, FileQuery};
use result::SearchResult;

pub fn run(args: &Vec<String>) -> Result<(), String>
{
    let path = check_args(args)?;

    let mut results: Vec<SearchResult> = Vec::new();

    if path.is_dir()
    {
        let query = DirQuery::new(args)?;
        results = match search_dir(query.path, query.search_str)
        {
            Ok(entries) => entries,
            Err(e) => panic!("{}", e),
        }
    }
    else if path.is_file()
    {
        let query = FileQuery::new(args)?;
        results = match search_file(query.search_str, query.name)
        {
            Ok(entries) => entries,
            Err(e) => panic!("{}", e),
        }
    }

    display(&results);

    Ok(())
}

fn display(results: &Vec<SearchResult>)
{
    if results.is_empty()
    {
        return;
    }
    else if results.len() == 1
    {
        println!("{}", results[0].filename.clone().bright_magenta().italic());
        println!("{}", results[0].format());
        return;
    }

    let mut current_file = results[1].filename.clone();

    for result in results.iter()
    {
        // Only print filenames if they are different from the
        // previously printed filename
        if result.filename.ne(&current_file)
        {
            current_file = result.filename.clone();
            println!("{}", current_file.bright_magenta().italic());
        }

        println!("{}", result.format());
    }
}

fn search_file(search_str: &str, name: &str) -> io::Result<Vec<SearchResult>>
{
    let contents = fs::read_to_string(name)?;
    let mut results: Vec<SearchResult> = Vec::new();
    let search_str = search_str;

    for (i, line) in contents.split('\n').enumerate()
    {
        for word in line.split(' ')
        {
            if word.contains(search_str)
            {
                // Indexes start from zero
                // so we need to add 1 to display the correct line numbers
                let result = SearchResult {
                    index: i + 1,
                    line: String::from(line),
                    search_str: String::from(search_str),
                    filename: String::from(name),
                };

                results.push(result);
            }
        }
    }

    Ok(results)
}

fn search_dir(search_path: &Path, search_str: &str) -> io::Result<Vec<SearchResult>>
{
    let mut results: Vec<SearchResult> = Vec::new();

    let (tx, rx) = mpsc::channel();

    for entry in fs::read_dir(search_path)?
    {
        let str_to_search = search_str.to_owned();
        // Create new transmitter for every searched file
        let new_tx = tx.clone();

        let sender = thread::spawn(move || {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir()
            {
                new_tx
                    .send(search_dir(&path, str_to_search.as_str()).unwrap())
                    .unwrap();
            }
            else
            {
                new_tx
                    .send(
                        search_file(str_to_search.as_str(), path.as_os_str().to_str().unwrap())
                            .unwrap(),
                    )
                    .unwrap();
            }
        });

        match rx.recv()
        {
            Ok(mut msg) => results.append(&mut msg),
            Err(_) => (), // No more messages are coming
        }

        sender.join().unwrap();
    }

    Ok(results)
}

fn check_args(args: &Vec<String>) -> Result<PathBuf, String>
{
    if args.len() != 3
    {
        return Err(format!(
            "Error: {} arguments were provided when 3 are required.",
            args.len()
        ));
    }

    let path = Path::new(&args[2]);

    if !path.exists()
    {
        return Err(format!("Error: {} does not exist.", path.to_str().unwrap()));
    }

    Ok(path.to_owned())
}

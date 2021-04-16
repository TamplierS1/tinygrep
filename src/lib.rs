#![feature(in_band_lifetimes)]

mod query;
mod result;

use std::fs;
use std::io;
use std::path::Path;

use colored::Colorize;

use query::DirQuery;
use query::FileQuery;
use result::SearchResult;

pub fn run(args: &Vec<String>)
{
    let path = Path::new(&args[2]);

    if !path.exists()
    {
        panic!("Error: {} does not exist.", path.display());
    }

    let mut results: Vec<SearchResult> = Vec::new();

    if path.is_dir()
    {
        let query = DirQuery::new(args);
        results = match search_dir(query)
        {
            Ok(entries) => entries,
            Err(e) => panic!("{}", e),
        }
    }
    else if path.is_file()
    {
        let query = FileQuery::new(args);
        results = match search_file(query.search_str, query.name)
        {
            Ok(entries) => entries,
            Err(e) => panic!("{}", e),
        }
    }

    display(&results);
}

fn display(results: &Vec<SearchResult>)
{
    // Print the first file's name
    if results.is_empty()
    {
        return;
    }

    let mut current_file = results[1].filename.clone();
    println!("{}", results[0].filename.clone().bright_magenta().italic());

    for result in results.iter()
    {
        // And now we compare against the second file's name
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
                let result = SearchResult {
                    index: i,
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

fn search_dir(query: DirQuery) -> io::Result<Vec<SearchResult>>
{
    let mut results: Vec<SearchResult> = Vec::new();

    for entry in fs::read_dir(query.path)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
        {
            results.append(&mut search_dir(query.clone())?);
        }
        else
        {
            results.append(&mut search_file(
                query.search_str,
                path.as_os_str().to_str().unwrap(),
            )?);
        }
    }

    Ok(results)
}

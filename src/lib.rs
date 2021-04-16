mod query;
mod result;

use query::Query;
use result::Result;

pub fn run(args: &Vec<String>)
{
    let query = Query::new(&args);

    let results = find(&query);
    display(&results);
}

fn display(results: &Vec<String>)
{
    for result in results.iter()
    {
        println!("{}", result);
    }
}

fn find(query: &Query) -> Vec<String>
{
    let mut results: Vec<String> = Vec::new();
    let search_str = query.search_str;
    for (i, line) in query.contents.split('\n').enumerate()
    {
        for word in line.split(' ')
        {
            if word.contains(search_str)
            {
                let result = Result {
                    index: i,
                    line,
                    word,
                    filename: query.filename,
                };

                results.push(result.format());
            }
        }
    }

    results
}

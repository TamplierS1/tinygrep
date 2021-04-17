use colored::*;

#[derive(Clone)]
pub struct SearchResult
{
    pub index: usize,
    pub line: String,
    pub search_str: String,
    pub filename: String,
}

impl SearchResult
{
    pub fn format(&self) -> String
    {
        // Filename is printed in 'tinygrep::display'
        let line_num = format!("{}", self.index).bright_green();

        // There is no need for pattern matching, we know that the line
        // contains the word
        let (before, after) = self.line.split_once(&self.search_str).unwrap();

        format!(
            "{}: {}{}{}",
            line_num,
            before.bright_white(),
            self.search_str.bright_red(),
            after.bright_white()
        )
    }
}

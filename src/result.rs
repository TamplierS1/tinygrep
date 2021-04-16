use colored::*;

pub struct Result<'a>
{
    pub index: usize,
    pub line: &'a str,
    pub word: &'a str,
    pub filename: &'a str,
}

impl Result<'_>
{
    pub fn format(&self) -> String
    {
        let filename = self.filename.bright_magenta().italic();
        let line_num = format!("{}:", self.index).bright_green();
        format!("{}\n{} {}", filename, line_num, self.line.bright_white())
    }
}

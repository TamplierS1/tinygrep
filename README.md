## tinygrep

`tinygrep` is a simple Rust implementation of `grep`. `tinygrep` is nowhere near as powerful as `grep` and is built only in an educational purpose.

## Dependencies

- [colored](https://crates.io/crates/colored) - 2.0.0 - colored terminal output.
- [twoway](https://crates.io/crates/twoway) - 0.2.1 - faster string searches.

## Usage

Search a single `file` for a `string`:  

`tinygrep string file`  

Search a `directory` for a `string` (recursively):  

`tinygrep string directory`

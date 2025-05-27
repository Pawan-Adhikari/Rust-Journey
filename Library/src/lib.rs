use std::io::{BufReader, BufRead};

pub mod color; // Declare the color module
pub mod config;

pub fn read_stdin()-> String
{
    let stdin = std::io::stdin();
    let mut line = String::new();
    let mut reader: BufReader<_> = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<M: BufRead>(reader: &mut M) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string() // Trim whitespace and return the line
}

#[cfg(test)]
mod tests {
    use crate::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_stdin() {
        let mut reader = Cursor::new("Hello, World!\n");
        let line = _read_stdin(&mut reader);
        assert_eq!(line, "Hello, World!");
    }
}
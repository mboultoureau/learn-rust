use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let count = count_words(&contents);

    println!("  {}  {} {} {}", count.lines, count.words, count.bytes, config.file_path);
    
    Ok(())
}

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub struct Count {
    pub lines: i32,
    pub words: i32,
    pub bytes: usize
}

pub fn count_words(contents: &str) -> Count {
    let mut line_count = 0;
    let mut word_count = 0;
    let mut previous_caracter: char = ' ';

    for c in contents.chars() {
        if c == '\n' {
            line_count += 1;
        }

        if (previous_caracter == ' ' || previous_caracter == '\n') && (c != ' ' && c != '\n') {
            word_count += 1;
        }

        previous_caracter = c;
    }

    Count {
        lines: line_count,
        words: word_count,
        bytes: contents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let contents = "\
Hello World!
This is a second line!";

        let count = count_words(contents);

        assert_eq!(1, count.lines);
        assert_eq!(7, count.words);
        assert_eq!(35, count.bytes);
    }
}
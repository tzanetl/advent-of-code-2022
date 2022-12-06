use std::fs;
use std::path::Path;

use log;
use simple_logger;

fn read_file(filepath: &Path) -> String {
    if filepath.exists() == false {
        panic!("Input file {:?} doesn't exist", filepath)
    }
    let content = fs::read_to_string(filepath).expect("unable to read message file");
    return content;
}

fn parse_input_file_path(args: &Vec<String>) -> &Path {
    if args.contains(&String::from("--test")) {
        return Path::new("test_input.txt")
    } else {
        return Path::new("input.txt")
    }
}

pub fn read_input(args: &Vec<String>) -> String {
    let filepath = parse_input_file_path(args);
    read_file(filepath)
}

pub fn set_logging_level(args: &Vec<String>) {
    let level: log::Level;
    if args.contains(&String::from("--test")) {
        level = log::Level::Debug;
    } else {
        level = log::Level::Info;
    }
    simple_logger::init_with_level(level).expect("Failed to init logger")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_file_path_test() {
        assert_eq!(
            parse_input_file_path(&vec![String::from("--test"),]),
            Path::new("test_input.txt")
        )
    }

    #[test]
    fn test_parse_input_file_path_not_test() {
        assert_eq!(
            parse_input_file_path(&vec![String::from("bwian"),]),
            Path::new("input.txt")
        )
    }
}

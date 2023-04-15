use std::fs;
use std::path::Path;
use std::sync::Once;

use log::{Record, Metadata};

static INIT_LOGGING: Once = Once::new();

static MY_LOGGER: MyLogger = MyLogger;

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
    }
    fn flush(&self) {}
}

fn read_file(filepath: &Path) -> String {
    if filepath.exists() == false {
        panic!("Input file {:?} doesn't exist", filepath)
    }
    let content = fs::read_to_string(filepath).expect("unable to read message file");
    return content;
}

pub fn parse_input_file_path(args: &Vec<String>) -> &Path {
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
    // https://stackoverflow.com/a/43093371/14536215
    INIT_LOGGING.call_once(|| {
        let level: log::LevelFilter;
        if args.contains(&String::from("--test")) {
            level = log::LevelFilter::Debug;
        } else {
            level = log::LevelFilter::Info;
        }
        log::set_logger(&MY_LOGGER).unwrap();
        log::set_max_level(level);
    });
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

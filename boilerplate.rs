use std::env;

use log::debug;

use utils::{read_input, set_logging_level};

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert!(true)
    }
}

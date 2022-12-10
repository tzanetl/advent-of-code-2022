use std::env;
use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::{VecDeque, HashSet};

use log::debug;

use utils::{parse_input_file_path, set_logging_level};


fn all_unique(buffer: &VecDeque<char>) -> bool {
    let mut set: HashSet<&char> = HashSet::new();
    for c in buffer {
        set.insert(c);
    }
    return set.len() == buffer.len();
}

fn parse_size(args: &Vec<String>) -> usize {
    match args.iter().position(|r| r == "--size") {
        Some(pos) => {
            args[pos + 1].parse::<usize>().unwrap()
        },
        None => 4
    }
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let filepath = parse_input_file_path(&args);

    let mut char_buffer: VecDeque<char> = VecDeque::new();
    let size: usize = parse_size(&args);
    let file = BufReader::new(File::open(filepath)?);
    println!("Set marker lenght with '--size N'");
    println!("Marker lenght: {size}");

    for (index, byte) in file.bytes().enumerate() {
        let b: u8 = byte.unwrap();
        // Break at CR / LF
        if (b == 13) | (b == 10) {
            break;
        }
        let c: char = b as char;

        if char_buffer.len() == size {
            char_buffer.pop_front();
        }
        char_buffer.push_back(c);
        debug!("{:?}", char_buffer);
        if all_unique(&char_buffer) & (char_buffer.len() == size) {
            println!("Message starts at: {}", index + 1);
            break;
        }
    }

    Ok(())
}

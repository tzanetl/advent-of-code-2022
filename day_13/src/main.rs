use std::env;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize)
}

impl Packet {
    pub fn add_packet(&mut self, p: Packet) {
        match self {
            Packet::Int(_) => panic!("Cannot add to Int"),
            Packet::List(list) => list.push(p)
        }
    }

    pub fn from_line(line: &str) -> Packet {
        let line_c: Vec<char> = line.chars().into_iter().collect();
        let (packet, _) = parse_list(&line_c[1..]);
        debug!("{:?}", packet);
        return packet;
    }
}

fn parse_list(mut input: &[char]) -> (Packet, &[char]) {
    let mut list = Packet::List(Vec::new());
    let mut packet: Packet;
    debug!("{:?}", input);

    loop {
        if input[0].is_ascii_digit() {
            (packet, input) = parse_int(input);
            list.add_packet(packet);
        } else if input[0] == '[' {
            (packet, input) = parse_list(&input[1..]);
            list.add_packet(packet);
        } else if input[0] == ',' {
            input = &input[1..];
        } else if input[0] == ']' {
            break;
        }
    }
    return (list, input);
}

fn parse_int(input: &[char]) -> (Packet, &[char]) {
    let mut int_string = String::new();
    let packet: Packet;
    let mut index: usize = 0;
    for (i, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            int_string.push(*c);
        } else {
            index = i;
            break;
        }
    }
    packet = Packet::Int(int_string.parse::<usize>().expect(
        &format!("Unable to parse Int from \"{}\"", int_string)[..]
    ));
    return (packet, &input[index..])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let mut lines = input.lines();
    loop {
        let left_line = lines.next();
        if left_line == None {
            break;
        }
        let left_packet = Packet::from_line(left_line.unwrap());
        let right_packet = Packet::from_line(lines.next().unwrap());

        // Skip empty line
        lines.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        let input = ['4', '2', ',', ']'];
        let (packet, remainder) = parse_int(&input);
        assert_eq!(packet, Packet::Int(42));
        assert_eq!(remainder, [',', ']']);
    }
}

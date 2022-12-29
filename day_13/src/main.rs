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
            return (list, &input[1..]);
        }
    }
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

    #[test]
    fn test_parse_packet_8_left() {
        set_logging_level(&vec!["--test".to_string()]);
        let packet = Packet::from_line("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        if let Packet::List(list) = packet {
            assert_eq!(list.last().unwrap(), &Packet::Int(9));
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_case_1 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[1,1,3,1,1]");
        let right_packet = Packet::from_line("[1,1,5,1,1]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_2 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[1],[2,3,4]]");
        let right_packet = Packet::from_line("[[1],4]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_3 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[9]");
        let right_packet = Packet::from_line("[[8,7,6]]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_4 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[4,4],4,4]");
        let right_packet = Packet::from_line("[[4,4],4,4,4]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_5 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[7,7,7,7]");
        let right_packet = Packet::from_line("[7,7,7]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_6 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[]");
        let right_packet = Packet::from_line("[3]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_7 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[[]]]");
        let right_packet = Packet::from_line("[[]]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_8 () {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right_packet = Packet::from_line("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        let ret = packet_to_packet(&left_packet, &right_packet);
        assert!(ret == false);
    }
}

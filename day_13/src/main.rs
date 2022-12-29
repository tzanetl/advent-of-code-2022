use std::env;
use std::fmt::Debug;
use std::cmp::Ordering;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(usize)
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("{} {}", self.size, self.name))
        match self {
            Packet::Int(value) => f.write_fmt(format_args!("{}", value)),
            Packet::List(list) => f.write_fmt(format_args!("{:?}", list))
        }
    }
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


fn int_to_int(left_value: &usize, right_value: &usize) -> Ordering {
    return left_value.cmp(right_value);
}

/// Are packets in order, i.e. is first non-equal value of left packet between packets smaller
fn in_order(left_packet: &Packet, right_packet: &Packet) -> bool {
    let order = packet_to_packet(left_packet, right_packet);
    match order {
        Ordering::Greater => return false,
        Ordering::Less => return true,
        Ordering::Equal => panic!("Cannot determine order")
    }
}

fn packet_to_packet(left_packet: &Packet, right_packet: &Packet) -> Ordering {
    let mut index: usize = 0;
    let (left_packet_list, right_packet_list) = match (left_packet, right_packet) {
        (Packet::List(left), Packet::List(right)) => (left, right),
        (_, _) => panic!("Both packets must be lists")
    };

    loop {
        let left_i = left_packet_list.get(index);
        let right_i = right_packet_list.get(index);
        debug!("left_i: {:?} right_i: {:?}", left_i, right_i);

        let (left, right) = match (left_i, right_i) {
            (Some(left), Some(right)) => (left, right),
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal
        };

        let ret: Ordering;
        match (left, right) {
            (Packet::Int(left_value), Packet::Int(right_value)) => {
                ret = int_to_int(left_value, right_value);
                debug!("int to int: {:?}", ret);
            },
            (Packet::List(_), Packet::Int(_)) => {
                ret = packet_to_packet(left, &Packet::List(vec![right.clone()]));
                debug!("list to int: {:?}", ret);
            },
            (Packet::Int(_), Packet::List(_)) => {
                ret = packet_to_packet(&Packet::List(vec![left.clone()]), right);
            },
            (Packet::List(_), Packet::List(_)) => {
                ret = packet_to_packet(left, right);
                debug!("list to list: {:?}", ret);
            }
        }

        match ret {
            Ordering::Equal => index += 1,
            _ => return ret
        }
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
    let mut packet_index: u32 = 1;
    let mut correctly_ordered: Vec<u32> = Vec::new();
    loop {
        let left_line = lines.next();
        if left_line == None {
            break;
        }
        let left_packet = Packet::from_line(left_line.unwrap());
        let right_packet = Packet::from_line(lines.next().unwrap());

        let was_in_order: bool = in_order(&left_packet, &right_packet);
        debug!("Packet order was correct: {:?}", was_in_order);

        if was_in_order {
            correctly_ordered.push(packet_index);
        }
        packet_index += 1;

        // Skip empty line
        lines.next();
    }

    let packet_index_sum: u32 = correctly_ordered.iter().sum();
    println!("correctly ordered packages: {:?}", correctly_ordered);
    println!("Packets analyzed: {}", packet_index - 1);
    println!("Sum of indeses of correctly ordered packages: {packet_index_sum}");
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
    fn test_case_1() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[1,1,3,1,1]");
        let right_packet = Packet::from_line("[1,1,5,1,1]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_2() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[1],[2,3,4]]");
        let right_packet = Packet::from_line("[[1],4]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_3() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[9]");
        let right_packet = Packet::from_line("[[8,7,6]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_4() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[4,4],4,4]");
        let right_packet = Packet::from_line("[[4,4],4,4,4]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_5() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[7,7,7,7]");
        let right_packet = Packet::from_line("[7,7,7]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_6() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[]");
        let right_packet = Packet::from_line("[3]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_7() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[[]]]");
        let right_packet = Packet::from_line("[[]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_8() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right_packet = Packet::from_line("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_9() {
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[[[5,0,9,4,6],8,2],1,0,[[9,4,3],[10,5,0,1],[]],0],[],[[[],9,4],[[2,8,3],6,0,[7,7,5]],[1,[10],8,[]]],[4,2,[4,5,[6,3,10,1,6],[10,3]]],[[[1,2],[0,1],7]]]");
        let right_packet = Packet::from_line("[[5,9],[6,[9],[1,[9,8,9,6],9,6,[1,5,7,6]],7,[3,[4,7,2],6,[5,9,4,10],[]]]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_10() {
        // Packet 16
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[4,[[],[2],[4,1,5,1],5,[2,7,5,7]]],[3,[],3,8],[[4,[4,7],[4,7],5,[5,3,1,5]],8,[1],9,6]]");
        let right_packet = Packet::from_line("[[[4]],[[[],0,[9,5,2,9,5]],9,[[8,1,7,5],10],6,[3,[5,2],8,[9,3,2,5]]],[[[],9,5,[0,7,8]]]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }

    #[test]
    fn test_case_11() {
        // Packet 65
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[10,5,5],[],[],[6,[[8]],0,[[2,4,9,1],4],[4,9,8,10,0]],[[[9,6,9,6]],8,[]]]");
        let right_packet = Packet::from_line("[[[10,[1,4,9,9],8,[6,10,6,8,10]],2,[1,6,1,5]],[],[[[2,3,8],[1],[10,8],10]]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == true);
    }

    #[test]
    fn test_case_12() {
        // Packet 67
        set_logging_level(&vec!["--test".to_string()]);
        let left_packet = Packet::from_line("[[[[10],[10,10],10,3],4],[7,[],5],[[],0,5,7],[[],[4,4,2,[8,0],0],[]],[2,[[],8,3,4],6,[[8,7,1,9],[0,0,0,5,9],5,4],[3]]]");
        let right_packet = Packet::from_line("[[10,[4,[9,5],[],7],3,[10,[7,7,3,5,4],9,[3,5,1,6,2]],[[8,3,10,6],8]]]");
        let ret = in_order(&left_packet, &right_packet);
        assert!(ret == false);
    }
}

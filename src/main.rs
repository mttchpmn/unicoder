extern crate core;


const HEADER4_MASK: u8 = 0x7;
const HEADER3_MASK: u8 = 0xF;
const HEADER2_MASK: u8 = 0x1F;
const HEADER1_MASK: u8 = 0x7F;
const TAIL_MASK: u8 = 0x3F;

fn main() {
    println!("Unicoder");

    let bytes = vec![0xF0, 0x9F, 0x98, 0x82];
    // let bytes = vec![0b1111_0000, 0b1001_1111, 0b1001_1000, 0b1000_0001];
    parse_octet_stream(bytes);
}

fn parse_octet_stream(stream: Vec<u8>) {
    let mut result: String = "".to_string();

    for octet in stream {

        let octet_type = get_octet_type(octet);

        match octet_type {
            OctetType::Header4 { data } => {
                result = "".to_string();
                let string = format!("{:0>3b}", data);
                result += &string;
            }
            OctetType::Header3 { data } => {
                result = "".to_string();
                let string1 = format!("{:0>4b}", data);
                result += &string1;
            }
            OctetType::Header2 { data } => {
                result = "".to_string();
                let string2 = format!("{:0>5b}", data);
                result += &string2;
            }
            OctetType::Header1 { data } => {
                result = "".to_string();
                let string3 = format!("{:0>7b}", data);
                result += &string3;
            }
            OctetType::Tail { data } => {
                let string4 = &format!("{:0>6b}", data);
                result += string4;
            }
        }
    }

    let num = u32::from_str_radix(&result, 2).unwrap();
    let char = char::from_u32(num).unwrap();

    println!("CHAR: {}", char);
}

fn get_octet_type(octet: u8) -> OctetType {
    match octet.leading_ones() {
        4 => {
            let data = octet & HEADER4_MASK;
            OctetType::Header4 { data }
        }
        3 => {
            let data = octet & HEADER3_MASK;
            OctetType::Header3 { data }
        }
        2 => {
            let data = octet & HEADER2_MASK;
            OctetType::Header2 { data }
        }
        1 => {
            let data = octet & TAIL_MASK;
            OctetType::Tail { data }
        }
        0 => {
            let data = octet & HEADER1_MASK;
            OctetType::Header1 { data }
        }
        _ => unreachable!("Not recognised")
    }
}

#[derive(Debug)]
pub enum OctetType {
    Header4 { data: u8 },
    Header3 { data: u8 },
    Header2 { data: u8 },
    Header1 { data: u8 },
    Tail { data: u8 },
}

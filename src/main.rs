extern crate core;


const HEADER4_MASK: u8 = 0x7;
const HEADER3_MASK: u8 = 0xF;
const HEADER2_MASK: u8 = 0x1F;
const HEADER1_MASK: u8 = 0x7F;
const TAIL_MASK: u8 = 0x3F;

fn main() {
    println!("Unicoder");

    // let bytes = vec![0xF0, 0x9F, 0x98, 0x82];
    let bytes = vec![0b1111_0000, 0b1001_1111, 0b1001_1000, 0b1000_0001];
    parse_octet_stream(bytes);
}

fn parse_octet_stream(stream: Vec<u8>) {
    let mut result: String = "".to_string();

    for octet in stream {
        println!("-----------------------------------");
        println!("OCTET: {} | {:b}b", octet, octet);
        // println!("RESULT: {} | {:b}b", result, result);
        dbg!(&result);

        let octet_type = get_octet_type(octet);

        match octet_type {
            OctetType::Header4 { data } => {
                result = "".to_string();
                // println!("DATA: {} | {:b}b", data, data);
                // result += data as u32;
                let string = format!("{:0>3b}", data);
                dbg!(&string);
                result += &string;
            }
            OctetType::Header3 { data } => {
                result = "".to_string();
                println!("DATA: {} | {:0b}b", data, data);
                // result += data as u32;
                let string1 = format!("{:0>4b}", data);
                dbg!(&string1);
                result += &string1;
            }
            OctetType::Header2 { data } => {
                result = "".to_string();
                println!("DATA: {} | {:0b}b", data, data);
                // result += data as u32;
                let string2 = format!("{:0>5b}", data);
                dbg!(&string2);
                result += &string2;
            }
            OctetType::Header1 { data } => {
                result = "".to_string();
                println!("DATA: {} | {:0b}b", data, data);
                // result += data as u32;
                let string3 = format!("{:0>7b}", data);
                dbg!(&string3);
                result += &string3;
            }
            OctetType::Tail { data } => {
                println!("DATA: {} | {:0b}b", data, data);
                // result += data as u32;
                let string4 = &format!("{:0>6b}", data);
                dbg!(&string4);
                result += string4;
            }
        }
    }

    println!("C: {}", result);

    let num = u32::from_str_radix(&result, 2).unwrap();

    let foo = char::from_u32(num).unwrap();

    println!("CHAR: {}", foo);
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

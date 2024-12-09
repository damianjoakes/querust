use std::fs::File;
use std::io::{BufReader, Read};
use crate::internal::buffer::Buffer;

#[test]
fn test_buffer() {
    let mut buf = Buffer::new(8192);
    let mut file = File::open(r#"./test.txt"#).unwrap();
    let _ = buf.read_some(&mut file).unwrap();
    let b = buf.buffer();
    dbg!(b);
    // let string = String::from_utf8_lossy_owned(b.to_vec());
    //
    // buf.discard_buffer();
    //
    // let _ = buf.read_some(&mut file).unwrap();
    // let b = buf.buffer();
    // let string2 = String::from_utf8_lossy_owned(b.to_vec());
    //
    // dbg!(string);
    // dbg!(string2);
}

#[test]
fn read_entire_file() {
    let mut buf = Buffer::new(16384);
    let mut file = File::open(r#"C:\\Users\damia\OneDrive\Documents\cmos-files\AP241127.H01"#).unwrap();

    while let Ok(bytes) = buf.read_some(&mut file) {
        if bytes == 0 { break; };

        let b = buf.buffer();
        let string = String::from_utf8_lossy_owned(b.to_vec());
        dbg!(string);

        if buf.end() >= buf.len() {
            buf.discard_buffer();
        }
    }
}

#[test]
fn buf_reader() {
    let mut file = File::open(r#"C:\\Users\damia\OneDrive\Documents\cmos-files\AP241127.H01"#).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    while let Ok(bytes) = reader.read_to_string(&mut buf) {
        if bytes == 0 { break; };
        dbg!(&buf);

        buf = String::new();
    }
}

#[test]
fn write_buffer() {
    let mut buf = Buffer::new(8192);
    rmp::encode::write_u8(&mut buf, 4).unwrap();
    dbg!(buf.buffer());

    let result = rmp::decode::read_u8(&mut buf.buffer()).unwrap();
    dbg!(result);
}
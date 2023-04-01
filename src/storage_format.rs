use std::{
    fs,
    mem,
    io::{self, BufReader}
};

use byteorder::{LittleEndian, ReadBytesExt};

mod errors;

pub(crate) use errors::MissmatchHeaderSize;

//TODO Implementation macros for parsing data
#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) magic_number: u16,
    pub(crate) names_size: i16,
    pub(crate) booleans_size: i16,
    pub(crate) numbers_size: i16,
    pub(crate) strings_size: i16,
    pub(crate) string_table_size: i16,
}

impl Header {
    pub(crate) fn new(data: &[u8]) -> Result<Self, MissmatchHeaderSize> {
        if data.len() < mem::size_of::<Header>() {
            return Err(MissmatchHeaderSize {
                required_size: mem::size_of::<Header>(),
                received_size: data.len(),
            });
        }

        let mut d_stream = BufReader::new(data);

        let magic_number = d_stream.read_u16::<LittleEndian>().unwrap();
        let names_size = d_stream.read_i16::<LittleEndian>().unwrap();
        let booleans_size = d_stream.read_i16::<LittleEndian>().unwrap();
        let numbers_size = d_stream.read_i16::<LittleEndian>().unwrap() * 2;
        let strings_size = d_stream.read_i16::<LittleEndian>().unwrap() * 2;
        let string_table_size = d_stream.read_i16::<LittleEndian>().unwrap();

        Ok(Self{
            magic_number,
            names_size,
            booleans_size,
            numbers_size,
            strings_size,
            string_table_size,
        })
    }
}

pub struct FileContent {
    pub(crate) header: Header,
    pub(crate) terminal_names: String,
    pub(crate) boolean_flags: Vec<i8>,
    pub(crate) numbers: Vec<i16>,
    pub(crate) strings_offset: Vec<i16>,
    pub(crate) string_table: Vec<u8>,
}

impl FileContent {

    //TODO change cast_vec to oth
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let content = fs::read(file_path)?;
        let header = match Header::new(&content) {
            Ok(header) => header,
            Err(err) => panic!("{}", err)
        };

        let header_size = mem::size_of::<Header>();

        let mut content: Vec<u8> = content.into_iter().skip(header_size).collect();
        let terminal_names: Vec<u8> = content.drain(..header.names_size as usize).collect();
        let terminal_names = String::from_utf8(terminal_names).expect("Uncorrect ASCII sequence");
        let boolean_flags: Vec<u8> = content.drain(..header.booleans_size as usize).collect();
        //let boolean_flags: Vec<i8> = vec![]; //tools::cast_vec(boolean_flags);
        let boolean_flags = boolean_flags.iter().map(
                |byte: &u8| -> i8 {
                    *byte as i8
                }).collect();
        let numbers: Vec<u8> = content.drain(..header.numbers_size as usize).collect();
        //let numbers: Vec<i16> = vec![]; //tools::cast_vec(numbers);
        let numbers = numbers.chunks(mem::size_of::<i16>()).map(
                |bytes: &[u8]| -> i16 {
                    BufReader::new(bytes).read_i16::<LittleEndian>().unwrap()
                }).collect();
        let strings_offset: Vec<u8> = content.drain(..header.strings_size as usize).collect();
        //let strings_offset: Vec<i16> = vec![]; //tools::cast_vec(strings_offset);
        let strings_offset = strings_offset.chunks(mem::size_of::<i16>()).map(
                |bytes: &[u8]| -> i16 {
                    BufReader::new(bytes).read_i16::<LittleEndian>().unwrap()
                }).collect();
        let string_table: Vec<u8> = content;

        Ok(Self {
            header,
            terminal_names,
            boolean_flags,
            numbers,
            strings_offset,
            string_table
        })
    }
}
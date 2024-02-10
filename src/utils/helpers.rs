use crate::config::*;
use crate::structs::limited_buffered_reader::LimitedBufferedReader;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

pub enum CustomReader {
    Limited(LimitedBufferedReader<BufReader<File>>),
    Regular(BufReader<File>),
}

pub fn build_reader(file: File) -> CustomReader {
    if USE_LIMITED_BUFFER {
        let inner = BufReader::new(file);
        CustomReader::Limited(LimitedBufferedReader::new(inner))
    } else {
        CustomReader::Regular(BufReader::new(file))
    }
}

impl Read for CustomReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            CustomReader::Limited(reader) => reader.read(buf),
            CustomReader::Regular(reader) => reader.read(buf),
        }
    }
}

impl BufRead for CustomReader {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        match self {
            CustomReader::Limited(reader) => reader.fill_buf(),
            CustomReader::Regular(reader) => reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            CustomReader::Limited(reader) => reader.consume(amt),
            CustomReader::Regular(reader) => reader.consume(amt),
        }
    }

    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        unsafe {
            match self {
                CustomReader::Limited(reader) => reader.read_until(b'\n', buf.as_mut_vec()),
                CustomReader::Regular(reader) => reader.read_until(b'\n', buf.as_mut_vec()),
            }
        }
    }
}

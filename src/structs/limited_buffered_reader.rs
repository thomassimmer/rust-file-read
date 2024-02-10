use crate::config::*;
use std::cmp::min;
use std::io::{BufRead, BufReader, ErrorKind, Read, Result};

pub struct LimitedBufferedReader<R> {
    reader: BufReader<R>,
}

impl<R: Read> LimitedBufferedReader<R> {
    pub fn new(inner: R) -> Self {
        LimitedBufferedReader {
            reader: BufReader::new(inner),
        }
    }
}

impl<R: Read> Read for LimitedBufferedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read> BufRead for LimitedBufferedReader<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        unsafe { self.read_until(b'\n', buf.as_mut_vec()) }
    }

    fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> Result<usize> {
        println!("Using our optimized method to read a line...");

        let mut read = 0;

        loop {
            let (done, used) = {
                let available = match self.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };

                match memchr::memchr(delim, available) {
                    Some(i) => {
                        if read <= NB_BYTES_ALLOWED_PER_LINE {
                            // buf here has DEFAULT_BUF_SIZE bytes allocated, but we shouldn't
                            // write more than NB_BYTES_ALLOWED_PER_LINE inside.
                            let max_bytes_to_write = min(NB_BYTES_ALLOWED_PER_LINE - read, i);
                            buf.extend_from_slice(&available[..max_bytes_to_write]);
                        }
                        (true, i + 1)
                    }
                    None => {
                        let available_len = available.len();
                        if read <= NB_BYTES_ALLOWED_PER_LINE {
                            let max_bytes_to_write =
                                min(NB_BYTES_ALLOWED_PER_LINE - read, available_len);
                            buf.extend_from_slice(&available[..max_bytes_to_write]);
                        }
                        (false, available_len)
                    }
                }
            };

            self.consume(used);
            read += used;

            if done || used == 0 {
                return Ok(read);
            }
        }
    }
}

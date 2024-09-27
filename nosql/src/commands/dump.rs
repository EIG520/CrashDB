use crate::data_types::data_types::*;

use std::{fmt, io::ErrorKind};
use super::commands::DbHandler;
use std::{fs::File, io::{BufWriter, BufRead, BufReader, Read, Write}, rc::Rc};

pub struct InvalidTypeSignature {signature: u8}

impl fmt::Debug for InvalidTypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid type signature {}", self.signature)
    }
}

impl fmt::Display for InvalidTypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid type signature {}", self.signature)
    }
}

impl std::error::Error for InvalidTypeSignature {}

impl DbHandler {
    pub fn handle_dump<'a>(&self, _: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = BufWriter::new(File::create(&self.dump_path)?);

        let mut bytes = 0;

        for (key, value) in self.data.lock().expect("couldn't mutex :(").iter() {

            let to_write = DbHandler::kv_bits(key, value);

            file.write_all(&to_write)?;

            bytes += to_write.len();

            file.flush()?;
        }

        Ok(format!("{} bytes dumped to {}", bytes, self.dump_path).to_bin().to_vec())
    }

    pub fn kv_bits(key: &String, value: &Rc<dyn Savable>) -> Vec<u8> {
        // entry structure:
        // | value bytes | value type | key | null byte | value |

        // 4 bytes for the "total bytes" section
        // so nothing longer than 2^32
  
        let key_bin = key.to_bin();
        let val_bin = value.to_bin();

        let size = val_bin.len();

        vec![
            // type signature
            value.signature(),

            // Number of bytes in value
            // bithacks 
                // evil if you're just seeing them for the first time
                // otherwise pretty self-explanatory
            ((size >> 24) & 255) as u8,
            ((size >> 16) & 255) as u8,
            ((size >> 8) & 255)  as u8,
            (size & 255)         as u8
        ]
            // the important part
            .iter()
                .chain(key_bin)
                .chain(&[0])
                .chain(val_bin)
                .map(|&i| i).collect::<Vec<u8>>()
    }

    pub fn handle_full_load(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if let Ok(file) = File::open(&self.dump_path) {
            let mut reader = BufReader::new(file);

            loop {
                // Get the type signature
                let mut tvec: Vec<u8> = vec![0];

                match reader.read_exact(&mut tvec) {
                    Ok(_) => {},
                    Err(e) => {
                        // stop if we reach eof
                        if e.kind() == ErrorKind::UnexpectedEof {break;}
                        else {return Err(Box::new(e));}
                    }
                }
                let vtype = tvec[0];

                // Get amount of bytes in value
                let mut bvec: Vec<u8> = vec![0; 4];
                reader.read_exact(&mut bvec)?;

                let val_bytes = ((bvec[0] as usize) << 24)
                    + ((bvec[1] as usize) << 16)
                    + ((bvec[2] as usize) << 8)
                    + bvec[3] as usize;


                // read until we find a null byte for key
                let mut kvec: Vec<u8> = vec![];
                reader.read_until(0, &mut kvec)?;
                kvec.pop();
                let key = String::from_bin(&kvec);

                // read "val_bytes" bytes for value
                let mut vvec: Vec<u8> = vec![0; val_bytes];
                reader.read_exact(&mut vvec)?;
                
                let value = match vtype {
                    0 => String::from_bin(&vvec),
                    signature => return Err(Box::new(InvalidTypeSignature {signature}))
                };

                self.save(key, Rc::new(value))?;
            }
            Ok(b"loaded".to_vec())
        } else {
            Ok(vec![])
        }
    }
}
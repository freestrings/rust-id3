use std::{io, vec};
use std::io::Result;
use std::io::{Read, Seek, SeekFrom};

pub struct Readable<I> where I: io::Read + io::Seek {
    input: I
}

impl<I> Readable<I> where I: io::Read + io::Seek {
    pub fn new(input: I) -> Self {
        Readable {
            input: input
        }
    }

    pub fn all_bytes(&mut self) -> Result<vec::Vec<u8>> {
        let mut buf = vec![];
        self.input.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn all_string(&mut self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.all_bytes()?).into_owned())
    }

    pub fn as_bytes(&mut self, amount: usize) -> Result<vec::Vec<u8>> {
        let mut buf = vec![0u8; amount];
        let read = self.input.read(buf.as_mut_slice())?;
        if read == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "read by 0"));
        }
        if read < amount {
            buf.split_off(read);
        }
        Ok(buf)
    }

    pub fn as_string(&mut self, amount: usize) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.as_bytes(amount)?).into_owned())
    }

    pub fn utf16_bytes(&mut self) -> Result<vec::Vec<u8>> {
        let mut ret = vec![];
        let mut read_all = 0;
        let mut buf = vec![0u8; 1];
        loop {
            let read = self.input.read(&mut buf)?;
            if read <= 0 {
                break;
            }
            read_all = read_all + read;
            if buf[0] == 0x00 {
                read_all = read_all + self.input.read(&mut buf)?;
                if buf[0] == 0x00 {
                    break;
                }
                ret.push(0x00);
                ret.push(buf[0]);
            } else {
                ret.push(buf[0]);
            }
        }
        Ok(ret)
    }

    // <text>0x00 0x00
    pub fn utf16_string(&mut self) -> Result<(usize, String)> {
        let ret = self.utf16_bytes()?;
        Ok((ret.len() + 2, String::from_utf8_lossy(&ret).into_owned()))
    }

    // <text>0x00
    pub fn non_utf16_bytes(&mut self) -> Result<vec::Vec<u8>> {
        let mut ret = vec![];
        let mut read_all = 0;
        let mut buf = vec![0u8; 1];
        loop {
            let read = self.input.read(&mut buf)?;
            if read <= 0 {
                break;
            }
            read_all = read_all + read;
            if buf[0] == 0x00 {
                break;
            } else {
                ret.push(buf[0]);
            }
        }
        Ok(ret)
    }

    pub fn non_utf16_string(&mut self) -> Result<(usize, String)> {
        let ret = self.non_utf16_bytes()?;
        Ok((ret.len() + 1, String::from_utf8_lossy(&ret).into_owned()))
    }

    pub fn skip(&mut self, amount: i64) -> Result<u64> {
        Ok(self.input.seek(SeekFrom::Current(amount))?)
    }

    pub fn position(&mut self, offset: u64) -> Result<u64> {
        Ok(self.input.seek(SeekFrom::Start(offset))?)
    }
}

pub mod factory {
    use std::{fs, io, vec};
    use std::io::Result;

    pub fn from_file(file: fs::File) -> Result<super::Readable<fs::File>> {
        Ok(super::Readable::new(file))
    }

    pub fn from_path(str: &str) -> Result<super::Readable<fs::File>> {
        Ok(super::Readable::new(fs::File::open(str)?))
    }

    pub fn from_str(str: &str) -> Result<super::Readable<io::Cursor<String>>> {
        Ok(super::Readable::new(io::Cursor::new(str.to_string())))
    }

    pub fn from_byte(bytes: vec::Vec<u8>) -> Result<super::Readable<io::Cursor<vec::Vec<u8>>>> {
        Ok(super::Readable::new(io::Cursor::new(bytes)))
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn bytes1() {
        let valid = "0123456789";
        if let Ok(mut readable) = super::factory::from_str(valid) {
            assert!(readable.as_bytes(10).is_ok());
            assert!(readable.as_bytes(10).is_err());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn byte2() {
        let str = "AB가나01".to_string();
        if let Ok(mut readable) = super::factory::from_byte(str.into_bytes()) {
            assert!(readable.skip(1).is_ok());
            assert_eq!(readable.as_string(1).unwrap(), "B");
            // utf8, 3bytes
            assert_eq!(readable.as_string(3).unwrap(), "가");
            assert_eq!(readable.as_string(5).unwrap(), "나01");
            assert!(readable.as_bytes(1).is_err());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn file1() {
        if let Ok(mut readable) = super::factory::from_path("./test-resources/file1.txt") {
            assert!(readable.as_bytes(10).is_ok());
            assert!(readable.as_bytes(10).is_ok());
            assert!(readable.skip(-5).is_ok());
            assert_eq!(readable.as_string(10).unwrap(), "fghij");
            assert!(readable.as_bytes(10).is_err());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn file2() {
        if let Ok(mut readable) = super::factory::from_path("./test-resources/file1.txt") {
            assert!(readable.skip(10).is_ok());
            assert!(readable.as_bytes(10).is_ok());
            assert!(readable.skip(-5).is_ok());
            assert_eq!(readable.as_string(10).unwrap(), "fghij");
            assert!(readable.as_bytes(10).is_err());
        } else {
            assert!(false);
        }
    }



    #[test]
    fn utf16_string() {
        let str = "AB가나01".to_string();
        let mut bytes: vec::Vec<u8> = str.into_bytes();
        bytes.push(0x00);
        bytes.push(0x01);
        bytes.push(0x00);
        bytes.push(0x00);
        bytes.push(0x02);
        assert_eq!(bytes.len(), 15);
        let mut readable = super::factory::from_byte(bytes).unwrap();
        let (size, read) = readable.utf16_string().unwrap();
        assert_eq!(size, 14);
        assert_eq!("AB\u{ac00}\u{b098}01\u{0}\u{1}", read);
        assert!(readable.skip(1).is_ok());
        assert!(readable.as_bytes(1).is_err());
    }
}
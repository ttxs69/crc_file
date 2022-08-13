use crc::{Crc, CRC_32_ISO_HDLC};
use crc_file::Config;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};

fn run(args: &[String]) -> Result<u32, Box<dyn Error>> {
    if args.len() != 4 {
        eprintln!("Usage: {} <filename> <offset> <size>", args[0]);
        return Err(Box::<dyn Error>::from(String::from("not enough arguments")));
    }

    let config = Config::new(&args).unwrap();
    let f = File::open(config.filename);
    if let Err(e) = f {
        eprintln!("Could not open file");
        return Err(Box::<dyn Error>::from(e));
    }

    let mut f = f.unwrap();
    let len = f.metadata()?.len();
    let offset = config.offset;
    let length = config.length;
    if offset + length > len {
        eprintln!("offset + length > filesize");
        return Err(Box::<dyn Error>::from(String::from(
            "offset + length > filesize",
        )));
    }

    let mut buf = vec![0; length as usize];
    f.seek(std::io::SeekFrom::Start(offset))?;
    f.read_exact(&mut buf)?;

    let crc_obj: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let crc = crc_obj.checksum(&buf);
    Ok(crc)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let crc = run(&args)?;
    println!("{:#x}", crc);
    Ok(())
}

#[cfg(test)]
mod test_main {
    use super::*;
    use assert_fs::prelude::*;
    #[test]
    fn test_not_enough_args() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from(file.path().to_str().unwrap()),
            String::from("0x0"),
        ];
        let result = run(&args).err().unwrap();
        assert!(result.to_string().contains("not enough arguments"));
    }

    #[test]
    fn test_over_size() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from(file.path().to_str().unwrap()),
            String::from("0x0"),
            String::from("0x1000"),
        ];
        let result = run(&args).err().unwrap();
        assert!(result.to_string().contains("offset + length > filesize"));
    }

    #[test]
    fn test_could_not_open_file() {
        let args = vec![
            String::from("crc_file"),
            String::from("file_not_exists.txt"),
            String::from("0x0"),
            String::from("0x1000"),
        ];
        let result = run(&args).err().unwrap();
        assert!(result.to_string().contains("No such file or directory"));
    }

    #[test]
    fn test_run_ok() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from(file.path().to_str().unwrap()),
            String::from("0x0"),
            String::from("10"),
        ];
        let result = run(&args).unwrap();
        assert_eq!(result, 0x672ec9d1);
    }
}

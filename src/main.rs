use crc_file::*;
use std::error;
fn main() -> Result<(), Box<dyn error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let config = Config::parse_args(args);
    if let Err(e) = config {
        if e.kind() == clap::ErrorKind::MissingRequiredArgument
            || e.kind() == clap::ErrorKind::DisplayHelp
            || e.kind() == clap::ErrorKind::DisplayVersion
        {
            return Ok(());
        } else {
            return Err(Box::new(e));
        }
    }
    let crc = run(&config.unwrap())?;
    println!("{:#x}", crc);
    Ok(())
}

#[cfg(test)]
mod test_main {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn test_over_size() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from(file.path().to_str().unwrap()),
            String::from("-o"),
            String::from("0x0"),
            String::from("-l"),
            String::from("0x1000"),
        ];
        let config = Config::parse_args(args).unwrap();
        let result = run(&config).err().unwrap();
        assert!(result.to_string().contains("offset + length > filesize"));
    }

    #[test]
    fn test_could_not_open_file() {
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from("file_not_exists.txt"),
            String::from("-o"),
            String::from("0x0"),
            String::from("-l"),
            String::from("0x1000"),
        ];
        let config = Config::parse_args(args).unwrap();
        let result = run(&config).err().unwrap();
        assert!(result.to_string().contains("No such file or directory"));
    }

    #[test]
    fn test_run_ok() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from(file.path().to_str().unwrap()),
            String::from("-o"),
            String::from("0x0"),
            String::from("-l"),
            String::from("10"),
        ];
        let config = Config::parse_args(args).unwrap();
        let result = run(&config).unwrap();
        assert_eq!(result, 0x672ec9d1);
    }

    #[test]
    fn test_run_with_only_filename() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("A test\nActual content\nMore content\nAnother test")
            .unwrap();
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from(file.path().to_str().unwrap()),
        ];
        let config = Config::parse_args(args).unwrap();
        let result = run(&config).unwrap();
        assert_eq!(result, 0xfcd8a8f2);
    }
}

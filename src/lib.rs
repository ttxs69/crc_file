use clap::command;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::error;
use std::fs::File;
use std::io::{Read, Seek};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub offset: u64,
    pub length: u64,
}

pub fn parse(str: &str) -> Result<u64, ParseIntError> {
    if str.starts_with("0x") {
        Ok(u64::from_str_radix(&&str[2..], 16)?)
    } else {
        Ok(u64::from_str_radix(&str, 10)?)
    }
}

pub fn run(config: &Config) -> Result<u32, Box<dyn error::Error>> {
    let f = File::open(&config.filename);
    if let Err(e) = f {
        eprintln!("Could not open file: {}", config.filename);
        return Err(Box::new(e));
    }

    let mut f = f.unwrap();
    let len = f.metadata()?.len();
    let offset = config.offset;
    let mut length = config.length;
    if offset + length > len {
        return Err(Box::<dyn error::Error>::from(String::from(
            "offset + length > filesize",
        )));
    }

    if length == 0 {
        length = len - offset;
    }

    let mut buf = vec![0; length as usize];
    f.seek(std::io::SeekFrom::Start(offset))?;
    f.read_exact(&mut buf)?;

    let crc_obj: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let crc = crc_obj.checksum(&buf);
    Ok(crc)
}

impl Config {
    pub fn parse_args(args: Vec<String>) -> Result<Config, clap::Error> {
        let mut cmd = command!() // requires `cargo` feature
            .arg(
                clap::Arg::new("filename")
                    .short('f')
                    .long("filename")
                    .display_order(1)
                    .takes_value(true),
            )
            .arg(
                clap::Arg::new("offset")
                    .short('o')
                    .long("offset")
                    .display_order(2)
                    .help("offset of the file to read")
                    .value_parser(clap::builder::ValueParser::new(parse))
                    .default_value("0"),
            )
            .arg(
                clap::Arg::new("length")
                    .short('l')
                    .long("length")
                    .display_order(3)
                    .help("length of the file to read, 0 means read to end of file")
                    .value_parser(clap::builder::ValueParser::new(parse))
                    .default_value("0"),
            );
        let matches = cmd.try_get_matches_from_mut(args);
        if let Err(e) = matches {
            match e.kind {
                clap::ErrorKind::DisplayHelp => {
                    cmd.print_help()?;
                    return Err(clap::Error::with_description(
                        String::from("No filename specified"),
                        clap::ErrorKind::DisplayHelp,
                    ));
                }
                clap::ErrorKind::DisplayVersion => {
                    println!("{}", cmd.render_long_version());
                    return Err(clap::Error::with_description(
                        String::from("No filename specified"),
                        clap::ErrorKind::DisplayVersion,
                    ));
                }
                _ => {
                    return Err(e);
                }
            }
        }
        let matches = matches.unwrap();
        let filename = matches.get_one::<String>("filename");
        if let Some(filename) = filename {
            let offset = *matches.get_one::<u64>("offset").unwrap();
            let length = *matches.get_one::<u64>("length").unwrap();
            Ok(Config {
                filename: filename.to_string(),
                offset,
                length,
            })
        } else {
            cmd.print_help()?;
            return Err(clap::Error::with_description(
                String::from("No filename specified"),
                clap::ErrorKind::MissingRequiredArgument,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::ErrorKind;

    use super::*;
    use std::{num::IntErrorKind, vec};
    #[test]
    fn test_parse() {
        assert_eq!(parse("0x123"), Ok(0x123));
        assert_eq!(parse("123"), Ok(123));
        assert_eq!(parse("0x").unwrap_err().kind(), &IntErrorKind::Empty);
        let result = parse("0x10000000000000000").unwrap_err();
        assert_eq!(result.kind(), &IntErrorKind::PosOverflow);
        let result = parse("abcdefg").unwrap_err();
        assert_eq!(result.kind(), &IntErrorKind::InvalidDigit);
        let result = parse("0xg").unwrap_err();
        assert_eq!(result.kind(), &IntErrorKind::InvalidDigit);
        let result = parse("").unwrap_err();
        assert_eq!(result.kind(), &IntErrorKind::Empty);
    }

    #[test]
    fn test_parse_args() {
        let args = vec![
            String::from("crc_file"),
            String::from("--filename"),
            String::from("test.txt"),
            String::from("--offset"),
            String::from("0x123"),
            String::from("--length"),
            String::from("0x456"),
        ];
        let config = Config::parse_args(args).unwrap();
        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.offset, 0x123);
        assert_eq!(config.length, 0x456);
    }

    #[test]
    fn test_parse_no_args() {
        let args = vec![String::from("crc_file")];
        let error = Config::parse_args(args).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_parse_wrong_arg() {
        let args = vec![String::from("crc_file"), String::from("test.txt")];
        let error = Config::parse_args(args);
        assert_eq!(error.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_parse_one_arg() {
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from("test.txt"),
        ];
        let config = Config::parse_args(args).unwrap();
        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.offset, 0);
        assert_eq!(config.length, 0);
    }

    #[test]
    fn test_parse_two_arg() {
        let args = vec![
            String::from("crc_file"),
            String::from("-f"),
            String::from("test.txt"),
            String::from("-o"),
            String::from("0x123"),
        ];
        let config = Config::parse_args(args).unwrap();
        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.offset, 0x123);
        assert_eq!(config.length, 0);
    }

    #[test]
    fn test_parse_args_help() {
        let args = vec![String::from("crc_file"), String::from("--help")];
        let error = Config::parse_args(args);
        assert_eq!(error.unwrap_err().kind(), ErrorKind::DisplayHelp);
        let args = vec![String::from("crc_file"), String::from("-h")];
        let error = Config::parse_args(args);
        assert_eq!(error.unwrap_err().kind(), ErrorKind::DisplayHelp);
    }
    #[test]
    fn test_parse_args_version() {
        let args = vec![String::from("crc_file"), String::from("--version")];
        let error = Config::parse_args(args);
        assert_eq!(error.unwrap_err().kind(), ErrorKind::DisplayVersion);
        let args = vec![String::from("crc_file"), String::from("-V")];
        let error = Config::parse_args(args);
        assert_eq!(error.unwrap_err().kind(), ErrorKind::DisplayVersion);
    }
}

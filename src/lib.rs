pub mod helper {
    use std::error::Error;
    pub fn parse(str: &str) -> Result<u64, Box<dyn Error>> {
        if str.starts_with("0x") {
            Ok(u64::from_str_radix(&&str[2..], 16)?)
        } else {
            Ok(u64::from_str_radix(&str, 10)?)
        }
    }
}

pub struct Config {
    pub filename: String,
    pub offset: u64,
    pub length: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        let offset = helper::parse(&args[2]).unwrap();
        let length = helper::parse(&args[3]).unwrap();
        let config = Config {
            filename,
            offset,
            length,
        };
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(helper::parse("0x12345").unwrap(), 0x12345);
        assert_eq!(helper::parse("12345").unwrap(), 12345);
    }

    #[test]
    fn test_parse_error() {
        assert!(helper::parse("0x12345g").is_err());
        assert!(helper::parse("12345a").is_err());
    }

    #[test]
    fn test_config() {
        let config = Config::new(&[
            String::from("crc_file"),
            String::from("test.txt"),
            String::from("0x0"),
            String::from("0x10"),
        ]);
        assert!(config.is_ok());
    }

    #[test]
    fn test_config_error() {
        let config = Config::new(&[
            String::from("crc_file"),
            String::from("test.txt"),
            String::from("0x0"),
        ]);
        assert!(config.is_err());
    }
}

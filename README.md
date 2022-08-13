# crc_file
![workflow](https://github.com/ttxs69/crc_file/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/ttxs69/crc_file/branch/main/graph/badge.svg?token=P8QMZZUR7Z)](https://codecov.io/gh/ttxs69/crc_file)

Calc a file's crc
## Usage
```
crc_file 0.1.0

USAGE:
    crc_file [OPTIONS]

OPTIONS:
    -f, --filename <filename>    
    -o, --offset <offset>        offset of the file to read [default: 0]
    -l, --length <length>        length of the file to read, 0 means read to end of file [default: 0]
    -h, --help                   Print help information
    -V, --version                Print version information
```
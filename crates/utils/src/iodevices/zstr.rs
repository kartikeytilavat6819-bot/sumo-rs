//! Stand-in for the zstr streams (foreign/zstr/zstr.hpp) that SUMO uses for gzipped files:
//! `zstr::ifstream` reads plain, gzip or zlib input, `zstr::ofstream` gzips when the name ends in ".gz".
use flate2::Compression;
use flate2::bufread::{MultiGzDecoder, ZlibDecoder};
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

/// opens a plain, gzip or zlib file like zstr::ifstream
pub fn open_read(path: &str) -> std::io::Result<Box<dyn BufRead>> {
    let mut f = BufReader::new(File::open(path)?);
    let head = f.fill_buf()?;
    let gz = head.starts_with(&[0x1f, 0x8b]);
    let zlib = head.len() >= 2 && head[0] == 0x78 && matches!(head[1], 0x01 | 0x9c | 0xda);
    Ok(if gz {
        Box::new(BufReader::new(MultiGzDecoder::new(f)))
    } else if zlib {
        Box::new(BufReader::new(ZlibDecoder::new(f)))
    } else {
        Box::new(f)
    })
}

/// opens a file for writing like zstr::ofstream: gzip-compressed when the name ends in ".gz"
pub fn create_write(path: &str) -> std::io::Result<Box<dyn Write>> {
    let file = File::create(path)?;
    Ok(if path.ends_with(".gz") {
        Box::new(GzEncoder::new(BufWriter::new(file), Compression::default()))
    } else {
        Box::new(BufWriter::new(file))
    })
}

#[cfg(test)]
mod tests {
    use super::{create_write, open_read};
    use std::io::{Read, Write};

    #[test]
    fn gz_and_plain_round_trip() {
        let dir = std::env::temp_dir().join(format!("sumo_rs_zstr_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let text = "10\n12\n13\n10 0 0 1 2\n";
        for name in ["table.txt", "table.gz"] {
            let path = dir.join(name);
            let p = path.to_str().unwrap();
            let mut w = create_write(p).unwrap();
            w.write_all(text.as_bytes()).unwrap();
            drop(w);
            let mut back = String::new();
            open_read(p).unwrap().read_to_string(&mut back).unwrap();
            assert_eq!(back, text);
        }
        let raw = std::fs::read(dir.join("table.gz")).unwrap();
        assert_eq!(&raw[..2], &[0x1f, 0x8b]);
        assert_eq!(std::fs::read(dir.join("table.txt")).unwrap(), text.as_bytes());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

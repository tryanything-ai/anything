use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};

pub fn trim_newline(s: &String) -> String {
    let mut s = s.clone();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

pub fn sha256_digest<R: Read>(mut reader: R) -> anyhow::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trims_newlines_from_string() {
        let s = String::from("hello\n");
        assert_eq!(trim_newline(&s), "hello");

        let s = String::from("hello\r\n");
        assert_eq!(trim_newline(&s), "hello");

        let s = String::from("hello");
        assert_eq!(trim_newline(&s), "hello");
    }
}

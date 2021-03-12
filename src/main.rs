use std::io::{self, Read, Write};

fn trim(buffer: &[u8]) -> &[u8] {
    let left = buffer.iter()
        .take_while(|&&x| x == b' ' || x == b'\n').count();
    let right = buffer.len() - buffer.iter().rev().take_while(|&&x| x == b' ' || x == b'\n').count();
    &buffer[left..right]
}

fn hex_decode(buffer: &[u8]) -> Vec<u8> {
    let buffer = trim(buffer);
    hex::decode(buffer).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();
    let mut buffer = String::new();
    stdin_lock.read_to_string(&mut buffer).unwrap();
    let decoded_string = hex_decode(buffer.as_bytes());
    stdout_lock.write(decoded_string.as_slice()).unwrap();
    stdout_lock.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        let s = b" 1234 \n";
        assert_eq!(trim(s), b"1234");
    }

    #[test]
    fn test_hex_decode() {
        let s = b"6369616f\n";
        assert_eq!(hex_decode(s), b"ciao");
    }
}
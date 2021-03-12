use std::io::{self, Read, Write};

fn hex_decode(buffer: &[u8]) -> Vec<u8> {
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

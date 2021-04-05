pub fn crlf(vec: &mut Vec<u8>) {
    vec.push(0x0D);
    vec.push(0x0A);
}

pub fn bitstring_as_chunks(bitstring: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for i in (0..bitstring.len()).step_by(512) {
        res.push(bitstring.to_owned()[i..i+512].to_string());
    }
    return res;
}

pub fn chunk_to_words(chunk: String, word_len: usize) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    for i in (0..chunk.len()).step_by(word_len) {
        res.push(u32::from_str_radix(&chunk[i..i+word_len], 2).unwrap());
    }
    return res;
}

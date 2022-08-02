pub fn msg_to_bitstring(message: String) -> String {
    let res: String = message.into_bytes().iter()
        .fold(String::new(), |acc, f| acc + format!("0{:b}", f).as_str());
    return res;
}

pub fn get_k(m_len: usize) -> u32 {
    for i in 0..512 {
        if (m_len + 1 + i + 64) % 512 == 0 {
            return i as u32;
        }
    }
    panic!("No suitable K found in pre-processing: Message lengh: {}", m_len);
}

/**
 * convert length of message to 64-bit bitstring
 */
pub fn len_to_bitstring(m_len: usize) -> String {
    let mut l: String = format!("0{:b}", m_len);
    if l.len() == 64 { 
        return l 
    } else if l.len() < 64 {
        let mut temp: String = String::new();
        (0..64 - l.len()).for_each(|_| temp.push('0'));
        temp.push_str(l.as_str());
        return temp;
    } else { 
        l = l[l.len() - 64 ..].to_string();
    }
    return l;
}

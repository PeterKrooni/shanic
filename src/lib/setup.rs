pub fn setup_payload(message: String) -> String{
    let mut bitstring: String = msg_to_bitstring(message);
    let m_len: usize = bitstring.len();
    bitstring.push('1');
    (0..get_k(m_len)).for_each(|_| bitstring.push('0'));
    bitstring.push_str(len_to_bitstring(m_len).as_str());
    return bitstring;
}

fn msg_to_bitstring(message: String) -> String {
    let res: String = message.into_bytes().iter()
        .fold(String::new(), |acc, f| acc + format!("0{:b}", f).as_str());
    return res;
}

fn get_k(m_len: usize) -> u32 {
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
fn len_to_bitstring(m_len: usize) -> String {
    let mut l: String = format!("0{:b}", m_len);
    match l.len().cmp(&64) {
        std::cmp::Ordering::Equal => {
            return l;
        }
        std::cmp::Ordering::Greater => {
            l = l[l.len() - 64 ..].to_string();
        }
        std::cmp::Ordering::Less => {
            let mut temp: String = String::new();
            (0..64 - l.len()).for_each(|_| temp.push('0'));
            temp.push_str(l.as_str());
            return temp;
        }
    }
    return l;
}

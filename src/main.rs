use std::time::SystemTime;

mod setup;
mod chunk;

const K_ARR: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

fn main() {
    let mut h_arr: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
    ];
    let mut input = String::new();
    println!("Enter a message to hash:");
    std::io::stdin().read_line(&mut input).expect("Something went wrong.");
    input = input.trim().to_string();

    let start = SystemTime::now();

    let mut bitstring = setup::msg_to_bitstring(input.to_owned());
    let m_len: usize = bitstring.len();
    bitstring.push('1');

    let k: u32 = setup::get_k(m_len);
    (0..k).for_each(|_| bitstring.push('0'));

    let l = setup::len_to_bitstring(m_len);
    bitstring.push_str(l.as_str());

    let chunks: Vec<String> = chunk::bitstring_as_chunks(bitstring);

    for c in chunks.iter() {
        let mut s: [u32; 64] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let words = chunk::chunk_to_words(c.to_owned(), 32);
        for w in 0..16 {
            s[w] = words[w];
        } 
        for i in 16..64 {
            let s0 = s[i - 15].rotate_right(7) ^ s[i - 15].rotate_right(18) ^ (s[i - 15] >> 3);
            let s1 = s[i - 2].rotate_right(17) ^ s[i - 2].rotate_right(19) ^ (s[i - 2] >> 10);
            s[i] = s[i - 16].wrapping_add(s0).wrapping_add(s[i - 7]).wrapping_add(s1);
        }
        let mut a = h_arr[0];
        let mut b = h_arr[1];
        let mut c = h_arr[2];
        let mut d = h_arr[3];
        let mut e = h_arr[4];
        let mut f = h_arr[5];
        let mut g = h_arr[6];
        let mut h = h_arr[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K_ARR[i]).wrapping_add(s[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let ma = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(ma);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h_arr[0] = h_arr[0].wrapping_add(a);
        h_arr[1] = h_arr[1].wrapping_add(b);
        h_arr[2] = h_arr[2].wrapping_add(c);
        h_arr[3] = h_arr[3].wrapping_add(d);
        h_arr[4] = h_arr[4].wrapping_add(e);
        h_arr[5] = h_arr[5].wrapping_add(f);
        h_arr[6] = h_arr[6].wrapping_add(g);
        h_arr[7] = h_arr[7].wrapping_add(h);
    }

    println!("\nDigest:");
    h_arr.iter().for_each(|i| print!("{}", format!("{:x}", i)));
    println!("\n\nTime taken: {:?}", SystemTime::now().duration_since(start));
}

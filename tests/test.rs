#[cfg(test)]
pub mod tests {
    use shanic::Shanic;

    #[test]
    fn empty() {
        let payload = "".to_owned();
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let mut sh: Shanic = Shanic { chunks: Vec::new() };
        Shanic::queue(&mut sh, payload);
        
        assert_eq!(Shanic::to_string(Shanic::get(&mut sh)), expected);
    }
    
    #[test]
    fn abc() {
        let payload = "abc".to_owned();
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

        let mut sh: Shanic = Shanic { chunks: Vec::new() };
        Shanic::queue(&mut sh, payload);
        
        assert_eq!(Shanic::to_string(Shanic::get(&mut sh)), expected);
    }

    #[test]
    fn big_input() {
        let mut payload = String::new();
        (0..1000).for_each(|_| payload.push('a'));
        let expected = "41edece42d63e8d9bf515a9ba6932e1c20cbc9f5a5d134645adb5db1b9737ea3";

        let mut sh: Shanic = Shanic { chunks: Vec::new() };
        Shanic::queue(&mut sh, payload);
        assert_eq!(Shanic::to_string(Shanic::get(&mut sh)), expected);   
    }
} 
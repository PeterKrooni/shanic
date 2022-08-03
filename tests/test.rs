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
        (0..1024).for_each(|_| payload.push('1'));
        let expected = "b4a38f5f5a21b6ba9f2008878beaa3a708866c66cbada71ba28ec359b5a53c3c";

        let mut sh: Shanic = Shanic { chunks: Vec::new() };
        Shanic::queue(&mut sh, payload);

        assert_eq!(Shanic::to_string(Shanic::get(&mut sh)), expected);   
    }
} 
use std::time::SystemTime;
use shanic::Shanic;

fn main() {
    let mut sh: Shanic = Shanic { chunks: Vec::new() };
    
    let start_empty = SystemTime::now();
    Shanic::queue(&mut sh, "".to_owned());
    let res_empty = Shanic::get(&mut sh).iter().fold(String::new(), |acc, f| acc + format!("{:x}", f).as_str());
    let end_empty = SystemTime::now();

    let start_abc = SystemTime::now();
    Shanic::queue(&mut sh, "abc".to_owned());
    let res_abc = Shanic::get(&mut sh).iter().fold(String::new(), |acc, f| acc + format!("{:x}", f).as_str());
    let end_abc = SystemTime::now();

    assert_eq!(res_empty, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"); 
    assert_eq!(res_abc, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"); 

    println!("--------------------\nBenchmark Result\n--------------------");
    println!("Duration, empty: \t{:?}", end_empty.duration_since(start_empty));
    println!("Duration, abc: \t\t{:?}", end_abc.duration_since(start_abc));

    println!("\nYour input:");
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Something went wrong.");

    Shanic::queue(&mut sh, input_string.trim().to_owned());
    println!("\nDigest:");
    println!("{}", Shanic::to_string(Shanic::get(&mut sh)));

}

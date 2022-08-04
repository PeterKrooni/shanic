use std::{time::SystemTime, time::Duration, env, process::exit};
use shanic::Shanic;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse(); args.pop(); args.reverse();
    let mut benchmarking = false;
    let mut bench_iter = 10000;
    if args.len() > 0 {
        match args[0].as_str() {
            "-b" => { benchmarking = true; }
            "-h" => {
                println!("Args:");
                println!("\t-b: <MAX, MIN, _> \t Benchmark algorithm; arguments are num iterations (10000000, 10000, 1000000)");
                println!("\t-h: \t\t\t Show this dialogue");
                exit(0);
            }
            _ => {}
        }
        if args.len() > 1 {
            match args[1].as_str() {
                "MAX" => { bench_iter = 10000000;}
                "MIN" => { bench_iter = 10000; }
                _     => { bench_iter = 1000000; }
            }
        }
    }

    let mut sh: Shanic = Shanic { chunks: Vec::new() };

    if benchmarking {         
        println!("--------------------\nBenchmark Result\n--------------------");
        Shanic::queue(&mut sh, "testval".to_owned());
        let mut timestamps: Vec<Duration> = Vec::new();
        for _ in 0..bench_iter {
            let n = SystemTime::now();
            let p = Shanic::get(&mut sh);
            match n.elapsed() {
                Ok(elapsed) => {
                    assert_eq!(Shanic::to_string(p), "93285be41b243afa17cc06e34495c4ed6d3c96c68b07ceb2340baa71cb5c417");
                    timestamps.push(elapsed);
                }
                Err(e) => {
                    panic!("Benchmarking error, {}", e);
                }
            }
        }
        let mut t: u128 = 0;
        let tll = timestamps.len() as u128;
        for i in timestamps {
            t += i.as_nanos();
        }
        let f: u128 = t / tll;
        println!("Average duration (ns): {}", f);
    } else {
        println!("Your input:");
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).expect("Something went wrong.");
    
        Shanic::queue(&mut sh, input_string.trim().to_owned());
        println!("Digest:");
        println!("{}", Shanic::to_string(Shanic::get(&mut sh)));
    }
}

# shanic

SHA-256 implementation, written in Rust. 
Pretty simple right now (only takes text input from CLI), but it would be fun to expand this to go faster and take large segments of data in short amounts of time. 

Producing a digest from an empty or short string currently takes about ~~100µs~~ 10µs

---

## How to run:
#### Requirements:
- Rust

###  Dev build:
- ```cargo build```
- ```cargo run```

### Release build:
- ```cargo build --release```
- ```./target/release/sha256```

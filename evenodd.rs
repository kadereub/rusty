// evenodd.rs
fn main() {
    for i in 0..8 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
}

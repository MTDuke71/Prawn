static NAME:&str = "prawn 0.1";
pub mod board;

fn main() {
    println!("Hello, {0}", NAME);
    board::board();
}
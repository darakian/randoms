use std::io;
use std::io::prelude::*;

fn main() {
    let mut test_array = [0u8; 1024*1024*4];
    let mut x = String::new();
    for i in 0..4{
        println!("Loop: {:?}. Press enter to continue", i);
        io::stdin().read_line(&mut x).expect("Error reading input");
        for j in i*1024..i*1024*1024{
            test_array[j] = i as u8;
        }
    }
}

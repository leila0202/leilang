use std::fs::{read_to_string};

fn main() {
    let filepath  = "test.lei";
    let content= read_to_string(filepath).unwrap();
    let lines = content.lines();
    for line in lines  {
        println!("|{}|", line);
        
    }
}

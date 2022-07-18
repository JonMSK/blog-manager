use std::io;

/// Wait For Input
pub fn wfi() {
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).unwrap();
}
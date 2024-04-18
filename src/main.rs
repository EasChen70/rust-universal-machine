use std::env;
use rumdump::um::rum;
use rumdump::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    rum(instructions);
    
}



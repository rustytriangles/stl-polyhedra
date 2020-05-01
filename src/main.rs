use std::env;

fn cube() {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        println!("Hello, {:?}!", args[i]);
    }

    let name = &args[1];
}

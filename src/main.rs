use std::{process, env};

fn main() {
    let ports = env::args().skip(1);

    if ports.len() < 1 {
        eprintln!("Need at least one argument!");
        process::exit(1);
    }

    let ports = ports.map(|port| {
        format!("-L {}:localhost:{}", port, port)
    });

    let ports = ports.collect::<Vec<_>>().join(" ");

    println!("{}", ports);
}

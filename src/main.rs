use std::env;
use std::time::Instant;
use wasmtime::{Engine, Module};

fn main() {
    println!("\n-------------------------------------");

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Module name(s) needed");
    }

    for file in args.iter().skip(1) {
        println!("\nLoading module {:?}", file);

        let start = Instant::now();
        let engine = Engine::default();
        let _module = Module::from_file(&engine, file).expect("Module failed to load");

        println!("Module {:?} loaded in {:?}", file, start.elapsed());
    }
}

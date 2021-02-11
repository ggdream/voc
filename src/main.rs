use std::{env, process, fs, io};
use rodio;


fn args() -> Vec<String> {
    env::args().collect()
}

fn play(path: &str) {
    let file = fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(io::BufReader::new(file)).unwrap();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    
    sink.append(source);
    sink.sleep_until_end()
}

fn main() {
    let a: Vec<String> = args();
    if a.len() != 2 {
        println!("Please type the true argv !!");
        process::exit(-1);
    }
    play(a.get(1).unwrap())
}
